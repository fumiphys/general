/*
 * author: fumiphys
 * this is a sample of ncurses, which is included in ncurses-rs examples.
 * add attribute to string like BOLD and BLINK
 */

extern crate ncurses;

use ncurses::*;
use std::char;

fn main() {
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();

    printw("Enter char");

    let ch = getch();
    if ch == KEY_F1 {
        attron(A_BOLD() | A_BLINK());
        printw("\nF1");
        attroff(A_BOLD() | A_BLINK());
        printw(" pressed");
    } else {
        attron(A_BOLD() | A_BLINK());
        printw(format!("\n{}", char::from_u32(ch as u32).expect("invalid")).as_ref());
        attroff(A_BOLD() | A_BLINK());
        printw(" pressed");
    }

    refresh();

    getch();
    endwin();
}
