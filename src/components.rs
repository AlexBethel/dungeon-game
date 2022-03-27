//! ECS components.

use specs::prelude::*;
use specs_derive::Component;

/// Entities that have a physical position in the world.
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Entities that need to be drawn as a single character.
#[derive(Component)]
pub struct CharRender {
    pub glyph: char,
}

/// Entities that users can control.
#[derive(Component)]
pub struct Player {
    /// The list of cells that are known to the player.
    pub known_cells: Vec<Vec<bool>>,
}

/// Entities that take turns periodically.
#[derive(Component)]
pub struct TurnTaker {
    /// Amount of time from now until the next scheduled turn.
    pub next: u32,

    /// Amount of time between turns.
    pub maximum: u32,
}

/// Entities that can move, attack other mobile entities, use items,
/// etc.
#[derive(Component)]
pub struct Mobile {
    pub next_action: MobAction,
}

/// Registers every existing component with the given ECS world.
pub fn register_all(world: &mut World) {
    world.register::<Position>();
    world.register::<CharRender>();
    world.register::<Player>();
    world.register::<TurnTaker>();
    world.register::<Mobile>();
}

impl From<&Position> for (i32, i32) {
    fn from(pos: &Position) -> Self {
        (pos.x, pos.y)
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

/// An action that a mob can perform that takes up a turn.
#[derive(Clone, Copy)]
pub enum MobAction {
    /// Do nothing.
    Nop,

    /// Physically move by the given vector.
    Move(i32, i32),
}
