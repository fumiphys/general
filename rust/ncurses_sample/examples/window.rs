/*
 * author: fumiphys
 * this is a sample of ncurses, which is included in ncurses-rs examples.
 * creating window
 */

extern crate ncurses;

use ncurses::*;

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;

fn create_win(start_y: i32, start_x: i32) -> WINDOW {
    let win = newwin(WINDOW_HEIGHT, WINDOW_WIDTH, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn destroy_win(win: WINDOW) {
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}

fn main() {
    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    printw("Hello World!");
    mvprintw(LINES() - 1, 0, "press q to exit");
    refresh();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
    let mut start_x = (max_x - WINDOW_WIDTH) / 2;
    let mut win = create_win(start_y, start_x);

    let mut ch = getch();
    while (ch as u32) != 113 {
        match ch {
            KEY_LEFT => {
                start_x -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            KEY_RIGHT => {
                start_x += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            KEY_UP => {
                start_y -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            KEY_DOWN => {
                start_y += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            _ => { }
        }
        ch = getch();
    }

    endwin();
}
