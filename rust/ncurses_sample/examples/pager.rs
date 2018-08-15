/*
 * author: fumiphys
 * this is a sample of ncurses, which is included in ncurses-rs examples.
 * paging
 */

extern crate ncurses;

use std::env;
use std::io::Read;
use std::fs;
use std::path::Path;
use ncurses::*;

fn open_file() -> fs::File {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("two arguments are required: {}", args[0]);
        panic!("Exiting...");
    }
    
    let reader = fs::File::open(Path::new(&args[1]));
    reader.ok().expect("failed to open file")
}

fn prompt() {
    printw("press any key");
    getch();
}

fn main() {

    let reader = open_file().bytes();

    initscr();
    keypad(stdscr(), true);
    noecho();

    // screen
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    for ch in reader {
        if ch.is_err() {break;}
        let ch = ch.unwrap();

        let mut cur_x = 0;
        let mut cur_y = 0;
        getyx(stdscr(), &mut cur_y, &mut cur_x);

        if cur_y == (max_y - 1) {
            prompt();

            clear();
            mv(0, 0);
        }

        addch(ch as chtype);
    }

    mv(max_y - 1, 0);
    prompt();

    endwin();
}
