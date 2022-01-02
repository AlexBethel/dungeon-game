//! ECS components.

use specs::prelude::*;
use specs_derive::Component;

/// Entities that have a physical position in the world.
#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Entities that need to be drawn as a single character.
#[derive(Component)]
pub struct CharRender {
    pub glyph: char,
}

/// Entities that the user can control using the keyboard.
#[derive(Component)]
pub struct Player;

/// Entities that take turns periodically.
#[derive(Component)]
pub struct TurnTaker {
    /// Amount of time from now until the next scheduled turn.
    pub next: u32,

    /// Amount of time between turns.
    pub maximum: u32,
}

/// Registers every existing component with the given ECS world.
pub fn register_all(world: &mut World) {
    world.register::<Position>();
    world.register::<CharRender>();
    world.register::<Player>();
    world.register::<TurnTaker>();
}
