use components::{register_all, CharRender, MobAction, Mobile, Player, Position, TurnTaker};
use io::init_window;
use level::{DungeonLevel, LEVEL_SIZE};

use player::player_turn;
use rand::thread_rng;
use specs::prelude::*;
use systems::{DiscoverySystem, MobSystem, TimeSystem};

mod components;
mod io;
mod level;
mod player;
mod rooms;
mod systems;
mod util;
mod visibility;

fn main() {
    let mut world = World::new();

    register_all(&mut world);

    let level = DungeonLevel::generate_level(&mut world, &mut thread_rng());
    let spawn_pos = level.upstairs[0];

    world.insert(level);

    world
        .create_entity()
        .with(Position::from(spawn_pos))
        .with(CharRender { glyph: '@' })
        .with(Player {
            known_cells: (0..LEVEL_SIZE.1)
                .map(|_| (0..LEVEL_SIZE.0).map(|_| false).collect())
                .collect(),
        })
        .with(Mobile {
            next_action: MobAction::Nop,
        })
        .with(TurnTaker {
            next: 0,
            maximum: 10,
        })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(TimeSystem, "time", &[])
        .with(MobSystem, "mobs", &[])
        .with(DiscoverySystem, "discovery", &[])
        .build();

    let mut window = match init_window() {
        Ok(window) => window,
        Err(err) => {
            println!("Error initializing window: {}", err);
            return;
        }
    };

    loop {
        dispatcher.dispatch(&world);

        if (
            &world.read_storage::<Player>(),
            &world.read_storage::<TurnTaker>(),
        )
            .join()
            .any(|(_plr, turn)| turn.next == 0)
        {
            player_turn(&mut world, &mut window);
        }
    }
}
