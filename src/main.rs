use components::{register_all, CharRender, Player, Position, TurnTaker};
use game::{BranchConfig, DungeonLevel};

use specs::prelude::*;
use systems::{IOSystem, TurnTickSystem};

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
        .with(Player)
        .with(TurnTaker {
            next: 0,
            maximum: 10,
        })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(TurnTickSystem, "turn_tick", &[])
        .with(IOSystem::new(), "render", &["turn_tick"])
        .build();

    loop {
        dispatcher.dispatch(&world);
    }
}
