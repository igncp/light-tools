use matches::MatchItem;
use ncurses::*;

// Relevant functions:
// mvprintw(LINES() - 1, 2, "Press F1 to exit");
//  printw("Use the arrow keys to move");

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;
static KEY_ENTER_FIXED: i32 = 10;

pub fn init_matches_ui(matched_items: Vec<MatchItem>) {
  initscr();
  raw();
  noecho();

  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

  // for match_item in &matched_items {
  // let full_str = format!("{} [{}/{}]\n", match_item.path, match_item.num, match_item.total);

  // printw(&full_str);
  // };
  //
  refresh();

  /* Get the screen bounds. */
  let mut max_x = 0;
  let mut max_y = 0;
  getmaxyx(stdscr(), &mut max_y, &mut max_x);

  /* Start in the center. */
  let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
  let mut start_x = (max_x - WINDOW_WIDTH) / 2;
  let mut win = create_win(start_y, start_x);

  let mut ch = getch();
  while ch != KEY_ENTER_FIXED {
    match ch {
      KEY_LEFT => {
        start_x -= 1;
        destroy_win(win);
        win = create_win(start_y, start_x);
      }
      KEY_RIGHT => {
        start_x += 1;
        destroy_win(win);
        win = create_win(start_y, start_x);
      }
      KEY_UP => {
        start_y -= 1;
        destroy_win(win);
        win = create_win(start_y, start_x);
      }
      KEY_DOWN => {
        start_y += 1;
        destroy_win(win);
        win = create_win(start_y, start_x);
      }
      _ => {
        let ch_str = format!("{} - {}\n", ch, KEY_ENTER);
        printw(&ch_str);
        refresh();
      }
    }
    ch = getch();
  }

  endwin();
}

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
