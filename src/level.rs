use std::fmt::Display;

use pancurses::Window;
use rand::{thread_rng, Rng};
use specs::prelude::*;

use crate::{
    components::{CharRender, Position},
    rooms,
};

/// The size of a dungeon level, in tiles.
pub const LEVEL_SIZE: (usize, usize) = (80, 24);

/// A single level of the dungeon.
#[derive(Clone)]
pub struct DungeonLevel {
    /// The tiles at every position in the level.
    tiles: [[DungeonTile; LEVEL_SIZE.0]; LEVEL_SIZE.1],

    /// The locations of the level's exits.
    exits: LevelExits,
}

/// The entrances and exits from a level.
#[derive(Clone)]
pub struct LevelExits {
    /// The location of each of the up-staircases.
    pub upstairs: Vec<(i32, i32)>,

    /// The location of each of the down-staircases.
    pub downstairs: Vec<(i32, i32)>,
}

/// The smallest measurable independent location in the dungeon,
/// corresponding to a single character on the screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DungeonTile {
    Floor,
    Wall,
    Hallway,
    Upstair,
    Downstair,
}

impl DungeonTile {
    /// Whether this tile is considered a floor tile, for the purposes
    /// of rendering walls.
    pub fn is_floor(&self) -> bool {
        match self {
            DungeonTile::Wall => false,
            DungeonTile::Hallway => false,
            _ => true,
        }
    }

    /// Whether this tile can be traveled through by normal
    /// creatures.
    pub fn is_navigable(&self) -> bool {
        self.is_floor() || self == &DungeonTile::Hallway
    }
}

impl DungeonLevel {
    /// Creates a new level with the given set of tiles, upstairs, and
    /// downstairs.
    pub fn new(
        tiles: [[DungeonTile; LEVEL_SIZE.0]; LEVEL_SIZE.1],
        upstairs: Vec<(i32, i32)>,
        downstairs: Vec<(i32, i32)>,
    ) -> Self {
        Self {
            tiles,
            exits: LevelExits {
                upstairs,
                downstairs,
            },
        }
    }

    /// Creates a new level and registers it with the given world.
    pub fn generate_level(world: &mut World) -> LevelExits {
        let level = rooms::generate_level(100, &mut thread_rng(), 1, 1);
        world.insert(level.clone()); // inefficient but whatever

        // Spawn some zombies in the world.
        for _ in 0..20 {
            let (x, y) = (
                thread_rng().gen_range(0..LEVEL_SIZE.0 as _),
                thread_rng().gen_range(0..LEVEL_SIZE.1 as _),
            );
            if level.tile(x, y).is_navigable() {
                world
                    .create_entity()
                    .with(Position { x, y })
                    .with(CharRender { glyph: 'Z' })
                    .build();
            }
        }

        level.exits
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
                // Walls are rendered like so:
                // - If the wall has any floor tiles to its north or
                //   south, then it is rendered as '-', because it is
                //   the north or south wall of a room.
                // - Otherwise, if the wall has any floor tiles to its
                //   east or west, then it is rendered as '|'.
                // - Otherwise, if any floor tiles are diagonally
                //   adjacent to the wall, then the wall is rendered as
                //   '+', because it is in the corner of a room.
                // - Otherwise, no floor tiles are adjacent to the
                //   wall, therefore it is surrounded by stone and will
                //   never be discovered by the player, so we don't
                //   render it at all.

                let has_floor = |deltas: &[(i32, i32)]| -> bool {
                    deltas
                        .iter()
                        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                        .filter(|(x, y)| {
                            (0..LEVEL_SIZE.0 as i32).contains(x)
                                && (0..LEVEL_SIZE.1 as i32).contains(y)
                        })
                        .any(|(x, y)| self.tile(x, y).is_floor())
                };

                if has_floor(&[(0, -1), (0, 1)]) {
                    '-'
                } else if has_floor(&[(-1, 0), (1, 0)]) {
                    '|'
                } else if has_floor(&[(-1, -1), (-1, 1), (1, -1), (1, 1)]) {
                    '+'
                } else {
                    ' '
                }
            }
            DungeonTile::Hallway => '#',
            DungeonTile::Upstair => '<',
            DungeonTile::Downstair => '>',
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
