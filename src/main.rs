use game::{BranchConfig, DungeonLevel};
use pancurses::{endwin, initscr};

mod game;
mod rooms;

fn main() {
    let window = initscr();

    let cfg = BranchConfig;
    let level = DungeonLevel::new(&cfg);

    level.draw(&window);
    window.refresh();
    window.getch();

    endwin();
}
