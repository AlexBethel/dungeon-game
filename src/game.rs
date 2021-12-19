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
#[derive(Debug, Clone, Copy)]
pub enum DungeonTile {
    Floor,
    Wall,
    Hallway,
}

impl DungeonLevel {
    /// Creates a new level in a branch that has the given
    /// configuration.
    pub fn new(cfg: &BranchConfig) -> Self {
        Self {
            tiles: rooms::generate_level(30, &mut rand::thread_rng()),
        }
    }

    /// Draws a level on the display window.
    pub fn draw(&self, win: &Window) {
        for (y, row) in self.tiles.iter().enumerate() {
            win.mv(y as _, 0);
            for tile in row {
                win.addch(match tile {
                    DungeonTile::Floor => '.',
                    DungeonTile::Wall => ' ',
                    DungeonTile::Hallway => '#',
                });
            }
        }

        // Leave the cursor at the lower-left.
        win.mv(0, 0);
    }
}
