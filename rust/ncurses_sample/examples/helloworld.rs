/*
 * author: fumiphys
 * this is a sample of ncurses, which is included in ncurses-rs examples.
 * printing hello world to display
 */

extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    printw("Hello World!");

    refresh();

    getch();
    endwin();
}
