//! ECS systems.

use std::sync::atomic::{AtomicBool, Ordering};

use lazy_static::lazy_static;
use pancurses::{endwin, initscr, Window};
use specs::prelude::*;

use crate::{
    components::{CharRender, Position},
    game::DungeonLevel,
};

/// System for drawing the state of the game, and potentially waiting
/// (blocking) for user input.
pub struct IOSystem {
    window: Window,
}

lazy_static! {
    static ref WINDOW_INITIALIZED: AtomicBool = AtomicBool::new(false);
}

impl IOSystem {
    pub fn new() -> Self {
        // See the note on `impl Send for IOSystem`.
        if WINDOW_INITIALIZED.swap(true, Ordering::Relaxed) {
            panic!("Refusing to initialize the renderer twice");
        }

        Self { window: initscr() }
    }
}

impl Drop for IOSystem {
    fn drop(&mut self) {
        endwin();
        WINDOW_INITIALIZED.store(false, Ordering::Relaxed);
    }
}

// The `Window` type from pancurses contains a raw pointer, and as a
// result Rust isn't convinced that it's safe to send between threads.
// Since we guarantee that only one `Window` object is ever created at
// a time, it is in fact safe to send the render system's data between
// threads.
unsafe impl Send for IOSystem {}

impl<'a> System<'a> for IOSystem {
    type SystemData = (
        ReadExpect<'a, DungeonLevel>,
        ReadStorage<'a, CharRender>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (level, renderables, positions): Self::SystemData) {
        // Draw the base level.
        level.draw(&self.window);

        // Draw all renderable entities in the ECS.
        for (render, pos) in (&renderables, &positions).join() {
            self.window.mvaddch(pos.y as _, pos.x as _, render.glyph);
        }

        // Leave the cursor at the lower-left.
        self.window.mv(0, 0);
        self.window.refresh();

        // For now, just get a character to avoid redrawing over and
        // over.
        self.window.getch();
    }
}
