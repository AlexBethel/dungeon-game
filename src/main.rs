use components::{register_all, CharRender, Position};
use game::{BranchConfig, DungeonLevel};

use specs::prelude::*;
use systems::IOSystem;

mod components;
mod game;
mod rooms;
mod systems;
mod util;

fn main() {
    let mut world = World::new();

    register_all(&mut world);

    let cfg = BranchConfig;
    let level = DungeonLevel::new(&cfg);

    world.insert(level);

    world
        .create_entity()
        .with(Position { x: 5, y: 6 })
        .with(CharRender { glyph: '@' })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(IOSystem::new(), "render_system", &[])
        .build();

    loop {
        dispatcher.dispatch(&world);
    }
}
