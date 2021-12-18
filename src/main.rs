use pancurses::{endwin, initscr};

fn main() {
    let window = initscr();
    window.printw("Hello World!");
    window.refresh();
    window.getch();
    endwin();
}
