use pancurses::Window;

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
    tiles: [[DungeonTile; LEVEL_SIZE.1]; LEVEL_SIZE.0],
}

/// The smallest possible independent location in the dungeon,
/// corresponding to a single character on the screen.
pub enum DungeonTile {
    Floor,
    Wall,
    Hallway,
}

impl DungeonLevel {
    /// Creates a new level in a branch that has the given
    /// configuration.
    pub fn new(cfg: &BranchConfig) -> Self {
        todo!()
    }

    /// Draws a level on the display window.
    pub fn draw(&self, win: &Window) {
        todo!()
    }
}
