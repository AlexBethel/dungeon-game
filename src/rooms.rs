//! Generator for levels that consist of a number of rooms connected
//! by hallways.

use std::ops::Range;

use grid::Grid;
use rand::Rng;

use crate::game::{DungeonTile, LEVEL_SIZE};

/// Generates a grid of the given size containing rooms connected by
/// passages.
pub fn generate(n_rooms: usize, size: (usize, usize), rng: &mut impl Rng) -> Grid<DungeonTile> {
    let mut grid = Grid::init(size.1, size.0, DungeonTile::Wall);
    let rooms = gen_room_bounds(n_rooms, size, rng);

    for room in rooms {
        for (x, y) in room.tiles() {
            grid[y][x] = DungeonTile::Floor;
        }
    }

    grid
}

/// Generates a grid of the statically-known level size.
pub fn generate_level(
    n_rooms: usize,
    rng: &mut impl Rng,
) -> [[DungeonTile; LEVEL_SIZE.0]; LEVEL_SIZE.1] {
    // FIXME: This function is atrocious. We do an allocation here
    // when we theoretically doesn't need to (we get a heap-allocated
    // Grid back, when we know statically that it's LEVEL_SIZE so we
    // could allocate it on the stack)...
    let grid = generate(n_rooms, LEVEL_SIZE, rng);

    // ...and then we use a pointless default of DungeonTile::Floor
    // here then copy in the real data from `grid`.
    let mut data = [[DungeonTile::Floor; LEVEL_SIZE.0]; LEVEL_SIZE.1];
    for (value, slot) in Iterator::zip(
        grid.into_vec().into_iter(),
        data.iter_mut().flat_map(|elem| elem.iter_mut()),
    ) {
        *slot = value;
    }

    data
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

    /// Returns whether the two rooms are overlapping.
    pub fn overlapping(&self, other: &Self) -> bool {
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
}

/// The possible sizes of a room, on both the x and y axes.
const ROOM_SIZE_LIMITS: Range<usize> = 4..8;

/// Generates bounds for a set of at most `n_rooms` nonoverlapping
/// rooms within a region of size `region_size`.
fn gen_room_bounds(
    n_rooms: usize,
    region_size: (usize, usize),
    rng: &mut impl Rng,
) -> Vec<RoomBounds> {
    let mut v: Vec<RoomBounds> = Vec::new();

    for _ in 0..n_rooms {
        let size = (
            rng.gen_range(ROOM_SIZE_LIMITS),
            rng.gen_range(ROOM_SIZE_LIMITS),
        );
        let ul_corner = (
            rng.gen_range(0..region_size.0 - size.0),
            rng.gen_range(0..region_size.1 - size.1),
        );

        let new_room = RoomBounds { ul_corner, size };
        if v.iter().all(|room| !room.overlapping(&new_room)) {
            v.push(new_room)
        }
    }

    v
}
