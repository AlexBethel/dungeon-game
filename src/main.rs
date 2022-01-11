use std::process::exit;

use components::{register_all, CharRender, MobAction, Mobile, Player, Position, TurnTaker};
use level::DungeonLevel;

use pancurses::{endwin, initscr, noecho, Window};
use player::player_turn;
use specs::prelude::*;
use systems::{MobSystem, TimeSystem};

mod components;
mod level;
mod player;
mod rooms;
mod systems;
mod util;

fn main() {
    let mut world = World::new();

    register_all(&mut world);

    let level = DungeonLevel::new();
    let spawn_pos = level.upstairs()[0];

    world.insert(level);

    world
        .create_entity()
        .with(Position {
            x: spawn_pos.0,
            y: spawn_pos.1,
        })
        .with(CharRender { glyph: '@' })
        .with(Player)
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
        .build();

    let mut window = init_window();

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

/// Initializes the terminal to accept user input, and creates a new
/// Window.
fn init_window() -> Window {
    // Create a new window over the terminal.
    let window = initscr();

    // Enable keypad mode (off by default for historical reasons), so
    // we can read special keycodes other than just characters.
    window.keypad(true);

    // Disable echoing so the user doesn't see flickering in the
    // upper-left corner of the screen when they type a character.
    noecho();

    window
}

/// Cleans everything up and exits the game.
fn quit() -> ! {
    endwin();

    exit(0)
}
