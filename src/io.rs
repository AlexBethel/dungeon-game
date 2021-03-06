//! Pancurses boilerplate code.

use std::process::exit;

use pancurses::{
    endwin, has_colors, init_pair, initscr, noecho, start_color, ColorPair, Window, COLORS,
    COLOR_PAIRS,
};
use thiserror::Error;

/// Initializes the terminal to accept user input, and creates a new
/// Window.
pub fn init_window() -> Result<Window, ColorError> {
    // Create a new window over the terminal.
    let window = initscr();

    // Enable keypad mode (off by default for historical reasons), so
    // we can read special keycodes other than just characters.
    window.keypad(true);

    // Disable echoing so the user doesn't see flickering in the
    // upper-left corner of the screen when they type a character.
    noecho();

    // Set up a color palette.
    init_colors()?;

    Ok(window)
}

/// Cleans everything up and exits the game.
pub fn quit() -> ! {
    endwin();

    exit(0)
}

/// The colors on a terminal.
#[allow(unused)]
pub enum Color {
    Black = pancurses::COLOR_BLACK as _,
    Red = pancurses::COLOR_RED as _,
    Green = pancurses::COLOR_GREEN as _,
    Yellow = pancurses::COLOR_YELLOW as _,
    Blue = pancurses::COLOR_BLUE as _,
    Magenta = pancurses::COLOR_MAGENTA as _,
    Cyan = pancurses::COLOR_CYAN as _,
    White = pancurses::COLOR_WHITE as _,
}

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("colors not supported")]
    NoColors,

    #[error("too few colors (have {0}, need 8)")]
    NotEnoughColors(u32),

    #[error("too few color slots (have {0}, need 8)")]
    NotEnoughSlots(u32),
}

fn init_colors() -> Result<(), ColorError> {
    assert_eq!(start_color(), 0);
    if !has_colors() {
        Err(ColorError::NoColors)
    } else if COLORS() < 8 {
        Err(ColorError::NotEnoughColors(COLORS() as _))
    } else if COLOR_PAIRS() < 8 {
        Err(ColorError::NotEnoughSlots(COLOR_PAIRS() as _))
    } else {
        for n in 0..8 {
            init_pair(n, n, Color::Black as _);
        }

        Ok(())
    }
}

pub fn set_color(win: &Window, c: Color) {
    // This gets called for every single character; could be a
    // performance bottleneck depending on how ncurses implements it.
    if has_colors() {
        win.attron(ColorPair(c as _));
    }
}
