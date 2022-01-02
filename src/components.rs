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

/// Registers every existing component with the given ECS world.
pub fn register_all(world: &mut World) {
    world.register::<Position>();
    world.register::<CharRender>();
}
