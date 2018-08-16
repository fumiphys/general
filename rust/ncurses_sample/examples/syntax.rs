/*
 * author: fumiphys
 * this is a sample of ncurses, which is included in ncurses-rs examples.
 * pager and syntax highlight
 */

extern crate ncurses;

use std::{ char, env, fs };
use std::path::Path;
use std::io::{ Read, Bytes };
use std::iter::Peekable;
use ncurses::*;

// color
static COLOR_BACKGROUND: i16 = 16;
static COLOR_FOREGROUND: i16 = 17;
static COLOR_KEYWORD: i16 = 18;
static COLOR_TYPE: i16 = 19;
static COLOR_STORAGE: i16 = 20;
static COLOR_COMMENT: i16 = 21;
static COLOR_STRING: i16 = 22;
static COLOR_CHAR: i16 = 23;
static COLOR_NUMBER: i16 = 24;

// color pair
static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_KEYWORD: i16 = 2;
static COLOR_PAIR_TYPE: i16 = 3;
static COLOR_PAIR_STORAGE: i16 = 4;
static COLOR_PAIR_COMMENT: i16 = 5;
static COLOR_PAIR_STRING: i16 = 6;
static COLOR_PAIR_CHAR: i16 = 7;
static COLOR_PAIR_NUMBER: i16 = 8;

static WORD_LIMITS: &'static [u8] = & [
    ' ' as u8,
    '(' as u8,
    ')' as u8,
    ':' as u8,
    ';' as u8,
    '&' as u8,
    '+' as u8,
    '-' as u8,
    ',' as u8,
    '.' as u8,
    '@' as u8,
    '~' as u8,
    '\\' as u8,
    '\n' as u8,
    '\r' as u8,
    '\0' as u8,
    !0 as u8,
];

struct Pager {
    file_reader: Peekable<Bytes<fs::File> >,

    in_comment: bool,
    in_string: bool,
    in_char: bool,

    screen_width: i32,
    screen_height: i32,
    curr_x: i32,
    curr_y: i32,
}

impl Pager {
    pub fn new() -> Pager {
        Pager {
            file_reader: open_file().bytes().peekable(),

            in_comment: false,
            in_string: false,
            in_char: false,

            screen_width: 0,
            screen_height: 0,
            curr_x: 0,
            curr_y: 0,
        }
    }

    pub fn initialize(&mut self) {
        initscr();
        keypad(stdscr(), true);
        noecho();

        start_color();
        init_color(COLOR_BACKGROUND, 0, 43 * 4, 54 * 4);
        init_color(COLOR_FOREGROUND, 142 * 4, 161 * 4, 161 * 4);
        init_color(COLOR_KEYWORD, 130 * 4, 151 * 4, 0);
        init_color(COLOR_TYPE, 197 * 4, 73 * 4, 27 * 4);
        init_color(COLOR_STORAGE, 219 * 4, 51 * 4, 47 * 4);
        init_color(COLOR_COMMENT, 33 * 4, 138 * 4, 206 * 4);
        init_color(COLOR_STRING, 34 * 4, 154 * 4, 142 * 4);
        init_color(COLOR_CHAR, 34 * 4, 154 * 4, 142 * 4);
        init_color(COLOR_NUMBER, 236 * 4, 107 * 4, 83 * 4);

        init_pair(COLOR_PAIR_DEFAULT, COLOR_FOREGROUND, COLOR_BACKGROUND);
        init_pair(COLOR_PAIR_KEYWORD, COLOR_KEYWORD, COLOR_BACKGROUND);
        init_pair(COLOR_PAIR_TYPE, COLOR_TYPE, COLOR_BACKGROUND);
        init_pair(COLOR_PAIR_STORAGE, COLOR_STORAGE, COLOR_BACKGROUND);
        init_pair(COLOR_PAIR_COMMENT, COLOR_COMMENT, COLOR_BACKGROUND);
        init_pair(COLOR_PAIR_STRING, COLOR_STRING, COLOR_BACKGROUND);
        init_pair(COLOR_PAIR_CHAR, COLOR_CHAR, COLOR_BACKGROUND);
        init_pair(COLOR_PAIR_NUMBER, COLOR_NUMBER, COLOR_BACKGROUND);

        bkgd(' ' as chtype | COLOR_PAIR(COLOR_PAIR_DEFAULT) as chtype);

        getmaxyx(stdscr(), &mut self.screen_height, &mut self.screen_width);
    }

    pub fn read_word(&mut self) -> (String, char) {
        let mut s: Vec<u8> = vec![];
        let mut ch: u8 = self.file_reader.next().unwrap().ok().expect("failed to read file");

        while !WORD_LIMITS.contains(&ch) {
            s.push(ch);
            ch = self.file_reader.next().unwrap().ok().expect("failed to read byte");
        }

        match char::from_u32(ch as u32) {
            Some(ch) => (String::from_utf8(s).ok().expect("failed to decode"), ch),
            None => (String::from_utf8(s).ok().expect("failed to decode"), ' '),
        }
    }

    pub fn highlight_word(&mut self, word: &str) -> attr_t {
        // comment
        if self.in_comment && !word.contains("*/") {
            return COLOR_PAIR(COLOR_COMMENT);
        } else if self.in_comment && word.contains("*/") {
            self.in_comment = false;
            return COLOR_PAIR(COLOR_PAIR_COMMENT);
        } else if !self.in_comment && word.contains("/*") {
            self.in_comment = true;
            return COLOR_PAIR(COLOR_PAIR_COMMENT);
        }
        // string
        if !self.in_char {
            if self.in_string && !word.contains("\"") {
                return COLOR_PAIR(COLOR_PAIR_STRING);
            } else if self.in_string && word.contains("\"") {
                self.in_string = false;
                return COLOR_PAIR(COLOR_STRING);
            } else if !self.in_string && word.contains("\"") {
                if word.find("\"") == word.rfind("\"") {
                    self.in_string = true;
                }
                return COLOR_PAIR(COLOR_STRING);
            }
        }
        // char
        if self.in_char && !word.contains("\'") {
            return COLOR_PAIR(COLOR_PAIR_CHAR);
        } else if self.in_char && word.contains("\'") {
            self.in_char = false;
            return COLOR_PAIR(COLOR_PAIR_CHAR);
        } else if !self.in_char && word.contains("\'") && !word.contains("static") {
            if word.find("\'") == word.rfind("\'") {
                self.in_char = true;
            }
            return COLOR_PAIR(COLOR_PAIR_CHAR);
        }

        let word = word.trim_matches(|ch: char| {
            WORD_LIMITS.contains(&(ch as u8))
        });

        if word.len() == 0 {
            return 0;
        }

        if word.as_bytes()[0] >= '0' as u8 && word.as_bytes()[0] <= '9' as u8 {
            return COLOR_PAIR(COLOR_PAIR_NUMBER);
        }

        match word {
             "break" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "continue" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "do" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "else" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "extern" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "in" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "if" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "impl" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "let" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "log" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "loop" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "match" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "once" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "priv" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "pub" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "return" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "unsafe" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "while" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "use" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "mod" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "trait" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "struct" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "enum" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "type" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },
             "fn" => { COLOR_PAIR(COLOR_PAIR_KEYWORD) },

             "int" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "uint" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "char" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "bool" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "u8" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "u16" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "u32" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "u64" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "i16" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "i32" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "i64" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "f32" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "f64" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "str" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "self" => { COLOR_PAIR(COLOR_PAIR_TYPE) },
             "Self" => { COLOR_PAIR(COLOR_PAIR_TYPE) },

             "const" => { COLOR_PAIR(COLOR_PAIR_STORAGE) },
             "mut" => { COLOR_PAIR(COLOR_PAIR_STORAGE) },
             "ref" => { COLOR_PAIR(COLOR_PAIR_STORAGE) },
             "static" => { COLOR_PAIR(COLOR_PAIR_STORAGE) },

             _ => 0,
        }
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        mv(self.screen_height - 1, 0);
        prompt();
        endwin();
    }
}

fn main() {
    let mut pager = Pager::new();
    pager.initialize();

    while pager.file_reader.peek().is_some() {
        let (word, leftover) = pager.read_word();
        let attr = pager.highlight_word(word.as_ref());
        let leftover_attr = pager.highlight_word(format!("{}", leftover).as_ref());

        getyx(stdscr(), &mut pager.curr_y, &mut pager.curr_x);

        if pager.curr_y == pager.screen_height - 1 {
            prompt();

            clear();
            mv(0, 0);
        } else {
            attron(attr);
            printw(word.as_ref());
            attroff(attr);

            attron(leftover_attr);
            addch(leftover as chtype);
            attroff(leftover_attr);
        }
    }
}

fn prompt() {
    attron(A_BOLD());
    printw("press space");
    while getch() != ' ' as i32 {}
    attroff(A_BOLD());
}

fn open_file() -> fs::File {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("two arguments are required");
        panic!("exiting...")
    }
    let reader = fs::File::open(Path::new(&args[1]));
    reader.ok().expect("failed to open file")
}
