use pancurses::Window;

pub struct Dungeon {
    main_branch: DungeonBranch,
}

pub struct DungeonBranch {
    config: BranchConfig,
    levels: Vec<DungeonLevel>,
}

pub struct BranchConfig;

pub const LEVEL_SIZE: (usize, usize) = (80, 24);

pub struct DungeonLevel {
    tiles: [[DungeonTile; LEVEL_SIZE.1]; LEVEL_SIZE.0],
}

pub enum DungeonTile {
    Floor,
    Wall,
    Hallway,
}

impl DungeonLevel {
    pub fn new(cfg: &BranchConfig) -> Self {
        todo!()
    }

    pub fn draw(&self, win: &Window) {
        todo!()
    }
}
