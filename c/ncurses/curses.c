#include <ncurses.h>

int main(){

  initscr();

  start_color();
  init_pair(1, COLOR_RED, COLOR_GREEN);
  bkgd(COLOR_PAIR(1));

  erase();
  move(10, 20);
  addstr("this is a curses sample");

  refresh();

  timeout(-1);
  getch();

  endwin();

  return 0;
}
