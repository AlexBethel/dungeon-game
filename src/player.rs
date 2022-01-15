//! Code for controlling the player, and for I/O.

use pancurses::Window;
use specs::prelude::*;

use crate::{
    components::{CharRender, MobAction, Mobile, Player, Position},
    level::DungeonLevel,
    quit,
};

/// Runs a player turn on the ECS, using the given `screen` for input
/// and output.
///
/// At some point this should maybe become a system rather than a
/// standalone function.
pub fn player_turn(ecs: &mut World, screen: &mut Window) {
    render_screen(ecs, screen);

    let action = loop {
        let key = screen.getch();

        use pancurses::Input;
        let action = match key {
            Some(key) => match key {
                Input::Character(ch) => match ch {
                    '.' => Some(MobAction::Nop),

                    'h' => Some(MobAction::Move(-1, 0)),
                    'j' => Some(MobAction::Move(0, 1)),
                    'k' => Some(MobAction::Move(0, -1)),
                    'l' => Some(MobAction::Move(1, 0)),

                    'y' => Some(MobAction::Move(-1, -1)),
                    'u' => Some(MobAction::Move(1, -1)),
                    'b' => Some(MobAction::Move(-1, 1)),
                    'n' => Some(MobAction::Move(1, 1)),

                    'q' => quit(),

                    _ => None,
                },

                Input::KeyUp => Some(MobAction::Move(0, -1)),
                Input::KeyLeft => Some(MobAction::Move(-1, 0)),
                Input::KeyDown => Some(MobAction::Move(0, 1)),
                Input::KeyRight => Some(MobAction::Move(1, 0)),
                _ => None,
            },

            // User closed stdin.
            None => quit(),
        };

        if let Some(action) = action {
            if possible(ecs, &action) {
                break action;
            }
        }
    };

    let plrs = ecs.read_storage::<Player>();
    let mut mobs = ecs.write_storage::<Mobile>();
    for (_plr, mob) in (&plrs, &mut mobs).join() {
        mob.next_action = action;
    }
}

/// Checks whether an action is possible for the player to execute in
/// the given world.
fn possible(ecs: &World, action: &MobAction) -> bool {
    match action {
        MobAction::Nop => true,
        MobAction::Move(dx, dy) => {
            let players = ecs.read_storage::<Player>();
            let positions = ecs.read_storage::<Position>();
            let map = ecs.fetch::<DungeonLevel>();

            (&players, &positions)
                .join()
                .all(|(_plr, pos)| map.tile(pos.x + dx, pos.y + dy).is_navigable())
        }
    }
}

/// Renders the state of the world onto the screen.
fn render_screen(ecs: &mut World, screen: &mut Window) {
    // Draw the base level.
    let level = ecs.fetch::<DungeonLevel>();
    level.draw(screen);

    // Draw all renderable entities.
    let renderables = ecs.read_storage::<CharRender>();
    let positions = ecs.read_storage::<Position>();
    for (render, pos) in (&renderables, &positions).join() {
        screen.mvaddch(pos.y as _, pos.x as _, render.glyph);
    }

    // Leave the cursor on the player's position.
    let plrs = ecs.read_storage::<Player>();
    let pos = ecs.read_storage::<Position>();
    let (_plr, pos) = (&plrs, &pos).join().next().unwrap();
    screen.mv(pos.y, pos.x);

    screen.refresh();
}
