//! Generator for levels that consist of a number of rooms connected
//! by hallways.
//!
//! The basic strategy here is that we start off by making some number
//! of attempts to place rectangular rooms of random sizes and
//! positions within the region; of these attempts, we only keep those
//! that are spread some distance away from other existing rooms. We
//! then use a pathfinding algorithm to navigate from each room to the
//! one generated after it, leaving hallways and doors as we travel.
//! The pathfinding algorithm is weighted to try and travel through
//! existing rooms and hallways rather than cutting new hallways
//! through the stone to encourage rooms to connect to other rooms
//! near them, and it has some randomness added to its weights to
//! discourage long, linear hallways.

use std::ops::Range;

use grid::Grid;
use pathfinding::directed::astar::astar;
use rand::Rng;

use crate::{
    game::{DungeonLevel, DungeonTile, LEVEL_SIZE},
    util::NiceFloat,
};

/// The possible sizes of a room, on both the x and y axes.
const ROOM_SIZE_LIMITS: Range<usize> = 4..8;

/// The minimum distance between the interiors of 2 rooms. Should be
/// at least 1 to ensure that walls generate.
const ROOM_MIN_DISTANCE: usize = 4;

/// The minimum distance between the interior of a room and the edge
/// of the map. Should be at least 1 to ensure that all rooms have
/// walls.
const ROOM_MARGIN: usize = 2;

/// Factor to encourage routes to travel through existing rooms rather
/// than cutting new hallways. 0.0 very strongly encourages traveling
/// through rooms, 1.0 is indifferent to the existence of rooms, and
/// higher values discourage traveling through rooms (hallways will
/// wrap around rooms rather than enter them).
const ROOM_WEIGHT: f64 = 0.2;

/// Randomness factor to avoid straight lines in hallways.
const HALLWAY_RANDOMNESS: f64 = 0.6;

/// Generates a grid of the given size containing rooms connected by
/// passages.
pub fn generate(
    n_rooms: usize,
    size: (usize, usize),
    rng: &mut impl Rng,
    upstairs: usize,
    downstairs: usize,
) -> (Grid<DungeonTile>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut grid = Grid::init(size.1, size.0, DungeonTile::Wall);
    let rooms = RoomBounds::generate(n_rooms, size, rng);

    for room in rooms.iter() {
        for (x, y) in room.tiles() {
            grid[y][x] = DungeonTile::Floor;
        }
    }

    add_hallways(&mut grid, &rooms, rng);
    let (upstairs, downstairs) = add_stairs(&mut grid, upstairs, downstairs, rng);

    (grid, upstairs, downstairs)
}

/// Generates a grid of the statically-known level size.
pub fn generate_level(
    n_rooms: usize,
    rng: &mut impl Rng,
    upstairs: usize,
    downstairs: usize,
) -> DungeonLevel {
    // FIXME: This function is atrocious. We do an allocation here
    // when we theoretically doesn't need to (we get a heap-allocated
    // Grid back, when we know statically that it's LEVEL_SIZE so we
    // could allocate it on the stack)...
    let (grid, upstairs, downstairs) = generate(n_rooms, LEVEL_SIZE, rng, upstairs, downstairs);

    // ...and then we use a pointless default of DungeonTile::Floor
    // here then copy in the real data from `grid`.
    let mut data = [[DungeonTile::Floor; LEVEL_SIZE.0]; LEVEL_SIZE.1];
    for (value, slot) in Iterator::zip(
        grid.into_vec().into_iter(),
        data.iter_mut().flat_map(|elem| elem.iter_mut()),
    ) {
        *slot = value;
    }

    DungeonLevel::from_raw_parts(data, upstairs, downstairs)
}

/// The bounding box of a room.
struct RoomBounds {
    ul_corner: (usize, usize),
    size: (usize, usize),
}

impl RoomBounds {
    /// Iterates over the tiles contained within the room.
    pub fn tiles(&self) -> impl Iterator<Item = (usize, usize)> {
        let (x_min, y_min) = self.ul_corner;
        let (x_max, y_max) = (x_min + self.size.0, y_min + self.size.1);

        (y_min..y_max).flat_map(move |y| (x_min..x_max).map(move |x| (x, y)))
    }

    /// Returns whether the two rooms are overlapping, i.e., there
    /// exists at least one tile that is contained in both rooms.
    pub fn intersects(&self, other: &Self) -> bool {
        fn range_overlapping(a: Range<usize>, b: Range<usize>) -> bool {
            if a.start > b.start {
                range_overlapping(b, a)
            } else {
                a.end > b.start
            }
        }

        range_overlapping(
            self.ul_corner.0..self.ul_corner.0 + self.size.0,
            other.ul_corner.0..other.ul_corner.0 + other.size.0,
        ) && range_overlapping(
            self.ul_corner.1..self.ul_corner.1 + self.size.1,
            other.ul_corner.1..other.ul_corner.1 + other.size.1,
        )
    }

    /// Returns whether the two rooms are within distance `dist` of
    /// one another or intersecting.
    pub fn near(&self, other: &Self, dist: usize) -> bool {
        RoomBounds {
            size: (self.size.0 + dist, self.size.1 + dist),
            ..*self
        }
        .intersects(&RoomBounds {
            size: (other.size.0 + dist, other.size.1 + dist),
            ..*other
        })
    }

    /// Generates bounds for a set of at most `n_rooms` nonoverlapping
    /// rooms within a region of size `region_size`.
    fn generate(n_rooms: usize, region_size: (usize, usize), rng: &mut impl Rng) -> Vec<Self> {
        let mut v: Vec<Self> = Vec::new();

        for _ in 0..n_rooms {
            let size = (
                rng.gen_range(ROOM_SIZE_LIMITS),
                rng.gen_range(ROOM_SIZE_LIMITS),
            );
            let ul_corner = (
                rng.gen_range(ROOM_MARGIN..region_size.0 - size.0 - ROOM_MARGIN),
                rng.gen_range(ROOM_MARGIN..region_size.1 - size.1 - ROOM_MARGIN),
            );

            let new_room = Self { ul_corner, size };
            if v.iter()
                .all(|room| !room.near(&new_room, ROOM_MIN_DISTANCE))
            {
                v.push(new_room)
            }
        }

        v
    }

    /// Calculates the approximate center of a room.
    fn center(&self) -> (usize, usize) {
        (
            self.ul_corner.0 + self.size.0 / 2,
            self.ul_corner.1 + self.size.1 / 2,
        )
    }
}

/// Adds a set of hallways connecting the given rooms to a dungeon.
fn add_hallways(grid: &mut Grid<DungeonTile>, rooms: &[RoomBounds], rng: &mut impl Rng) {
    // How hard we try to avoid traveling through stone at a pair of
    // coordinates.
    let mut stone_weights = Grid::new(grid.rows(), grid.cols());
    for elem in stone_weights.iter_mut() {
        *elem = rng.gen_range(1.0 - HALLWAY_RANDOMNESS..1.0 + HALLWAY_RANDOMNESS);
    }

    let size = (grid.cols(), grid.rows());

    // Make hallways between pairs of adjacent rooms.
    for rooms in rooms.windows(2) {
        let (from, to) = (&rooms[0].center(), &rooms[1].center());
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let (path, _weight) = astar(
            from,
            |node| {
                let (x, y) = (node.0 as isize, node.1 as isize);
                neighbors
                    .iter()
                    .map(move |(dx, dy)| (x + dx, y + dy))
                    .filter_map(|(x, y)| {
                        if (0..size.0 as isize).contains(&x) && (0..size.1 as isize).contains(&y) {
                            Some((
                                (x as usize, y as usize),
                                NiceFloat(match grid[y as usize][x as usize] {
                                    DungeonTile::Wall => stone_weights[y as usize][x as usize],
                                    _ => ROOM_WEIGHT,
                                }),
                            ))
                        } else {
                            None
                        }
                    })
            },
            |node| {
                // For A* to work correctly, the heuristic here must
                // be smaller than the actual cost to travel from
                // `node` to `to`, which means we need to know the
                // minimum possible cost from `node` to `to`.

                // The minimum possible cost to travel through a
                // single node if it's a wall is 1.0 -
                // HALLWAY_RANDOMNESS, and if it's a hallway then it's
                // ROOM_WEIGHT.
                let min_node_cost = f64::min(1.0 - HALLWAY_RANDOMNESS, ROOM_WEIGHT);

                // And since hallways don't travel diagonally, the
                // minimum number of nodes to travel through is the
                // sum of the horizontal and vertical distance.
                let dx = node.0 as isize - to.0 as isize;
                let dy = node.1 as isize - to.1 as isize;
                let min_dist = dx.abs() + dy.abs();

                NiceFloat(min_dist as f64 * min_node_cost)
            },
            |node| node == to,
        )
        .expect("Grid is connected therefore should be navigable");

        for (x, y) in path {
            if grid[y][x] == DungeonTile::Wall {
                grid[y][x] = DungeonTile::Hallway;
            }
        }
    }
}

/// Adds staircases leading upwards and downwards to the level.
fn add_stairs(
    grid: &mut Grid<DungeonTile>,
    n_upstairs: usize,
    n_downstairs: usize,
    rng: &mut impl Rng,
) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let (mut upstairs, mut downstairs) = (
        Vec::with_capacity(n_upstairs),
        Vec::with_capacity(n_downstairs),
    );

    for _ in 0..n_upstairs {
        let (x, y) = empty_square(grid, rng);
        upstairs.push((x, y));
        grid[y as usize][x as usize] = DungeonTile::Upstair;
    }

    for _ in 0..n_downstairs {
        let (x, y) = empty_square(grid, rng);
        downstairs.push((x, y));
        grid[y as usize][x as usize] = DungeonTile::Downstair;
    }

    (upstairs, downstairs)
}

/// Finds an unoccupied (floor) square of the level.
fn empty_square(grid: &Grid<DungeonTile>, rng: &mut impl Rng) -> (i32, i32) {
    loop {
        let (x, y) = (rng.gen_range(0..grid.cols()), rng.gen_range(0..grid.rows()));

        if grid[y][x] == DungeonTile::Floor {
            break (x as _, y as _);
        }
    }
}
