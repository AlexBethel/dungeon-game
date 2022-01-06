use std::fmt::Display;

use pancurses::Window;

use crate::rooms;

/// A dungeon root.
pub struct Dungeon {
    main_branch: DungeonBranch,
}

/// A single branch of a dungeon, which has a number of levels and
/// which can potentially contain passages to other branches.
pub struct DungeonBranch {
    config: BranchConfig,
    levels: Vec<DungeonLevel>,
}

/// The parameters that characterize a particular dungeon branch.
/// Currently a unit struct because there's only one type of branch,
/// but will later include e.g. architectural styles, good vs. evil &
/// lawful vs. chaotic weights, etc.
pub struct BranchConfig;

/// The size of a dungeon level, in tiles.
pub const LEVEL_SIZE: (usize, usize) = (80, 24);

/// A single level of the dungeon.
pub struct DungeonLevel {
    /// The tiles at every position in the level.
    tiles: [[DungeonTile; LEVEL_SIZE.0]; LEVEL_SIZE.1],
}

/// The smallest measurable independent location in the dungeon,
/// corresponding to a single character on the screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DungeonTile {
    Floor,
    Wall,
    Hallway,
}

impl DungeonLevel {
    /// Creates a new level in a branch that has the given
    /// configuration.
    pub fn new(_cfg: &BranchConfig) -> Self {
        Self {
            tiles: rooms::generate_level(100, &mut rand::thread_rng()),
        }
    }

    /// Draws a level on the display window.
    pub fn draw(&self, win: &Window) {
        for y in 0..LEVEL_SIZE.1 {
            win.mv(y as _, 0);
            for x in 0..LEVEL_SIZE.0 {
                win.addch(self.render_tile(x, y));
            }
        }
    }

    /// Renders the tile at the given coordinates.
    pub fn render_tile(&self, x: usize, y: usize) -> char {
        match self.tiles[y][x] {
            DungeonTile::Floor => '.',
            DungeonTile::Wall => {
                // Don't render walls with no adjacent floor space, to
                // keep the screen clear. Aside from that, walls are
                // '-' by default, unless the only adjacent floor is
                // to the east or west, in which case they are '|'.

                let neighborhood = (-1..=1)
                    .flat_map(|row| (-1..=1).map(move |col| (col, row)))
                    .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
                    .filter_map(|(dx, dy)| {
                        let (x, y) = (
                            usize::try_from(x as isize + dx).ok()?,
                            usize::try_from(y as isize + dy).ok()?,
                        );
                        Some((x, y, self.tiles.get(y)?.get(x)?))
                    })
                    .collect::<Vec<(usize, usize, &DungeonTile)>>();

                if neighborhood
                    .iter()
                    .all(|(_x, _y, tile)| *tile != &DungeonTile::Floor)
                {
                    ' '
                } else if neighborhood
                    .iter()
                    .any(|(tile_x, _y, tile)| *tile_x == x && *tile == &DungeonTile::Floor)
                {
                    '-'
                } else {
                    '|'
                }
            }
            DungeonTile::Hallway => '#',
        }
    }

    /// Gets a reference to the tile at the given coordinates. Panics
    /// of the coordinates are out of bounds.
    pub fn tile(&self, x: i32, y: i32) -> &DungeonTile {
        &self.tiles[y as usize][x as usize]
    }
}

impl Display for DungeonLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..LEVEL_SIZE.1 {
            for x in 0..LEVEL_SIZE.0 {
                write!(f, "{}", self.render_tile(x, y))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
