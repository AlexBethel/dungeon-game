//! ECS systems.

use specs::prelude::*;

use crate::components::{MobAction, Mobile, Position, TurnTaker};

/// System for ticking the turn counter on every entity; this system
/// implements the relationship between real-world time and in-game
/// time.
pub struct TimeSystem;

impl<'a> System<'a> for TimeSystem {
    type SystemData = WriteStorage<'a, TurnTaker>;

    fn run(&mut self, mut turn_takers: Self::SystemData) {
        for ent in (&mut turn_takers).join() {
            ent.next = ent.next.checked_sub(1).unwrap_or(ent.maximum);
        }
    }
}

/// System for executing actions that mobs have chosen.
pub struct MobSystem;

impl<'a> System<'a> for MobSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, TurnTaker>,
        WriteStorage<'a, Mobile>,
    );

    fn run(&mut self, (mut pos, turn, mut mob): Self::SystemData) {
        for (pos, _turn, mob) in (&mut pos, &turn, &mut mob)
            .join()
            .filter(|(_pos, turn, _mob)| turn.next == 0)
        {
            match mob.next_action {
                MobAction::Nop => {}
                MobAction::Move(dx, dy) => {
                    pos.x = (pos.x as i32 + dx) as _;
                    pos.y = (pos.y as i32 + dy) as _;
                }
            }

            mob.next_action = MobAction::Nop;
        }
    }
}
