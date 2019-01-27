use matches::MatchItem;
use ncurses::*;
use std::cmp::min;

static KEY_Q: i32 = 113;
static MATCHES_WINDOW_TOP_POS: i32 = 6;

struct CreateWindowOpts {
  height: i32,
  start_x: i32,
  start_y: i32,
  width: i32,
}

struct UIWindow {
  orig_window: WINDOW,
  width: i32,
}

impl UIWindow {
  fn print_line(&self, line: &String, y_pos: i32) {
    let chars_len = min((&self).width as usize - 1, line.len() - 1);
    let sliced_str = line.chars().take(chars_len).collect::<String>();
    mvwprintw((&self).orig_window, y_pos, 1, &sliced_str);
    wrefresh((&self).orig_window);
  }
}

fn create_window(opts: &CreateWindowOpts) -> UIWindow {
  let win = newwin(opts.height, opts.width, opts.start_y, opts.start_x);
  box_(win, 0, 0);
  wrefresh(win);

  UIWindow {
    orig_window: win,
    width: opts.width,
  }
}

fn get_screen_dimensions() -> (i32, i32) {
  let mut max_x = 0;
  let mut max_y = 0;
  getmaxyx(stdscr(), &mut max_y, &mut max_x);

  (max_x, max_y)
}

fn destroy_window(win: WINDOW) {
  let ch = ' ' as chtype;

  wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
  wrefresh(win);

  delwin(win);
}

fn create_matches_window(screen_width: i32, window_height: i32) -> UIWindow {
  create_window(&CreateWindowOpts {
    height: window_height,
    width: screen_width,
    start_x: 0,
    start_y: MATCHES_WINDOW_TOP_POS,
  })
}

fn create_title_window(screen_width: i32) -> UIWindow {
  create_window(&CreateWindowOpts {
    height: MATCHES_WINDOW_TOP_POS - 1,
    width: screen_width,
    start_x: 0,
    start_y: 1,
  })
}

pub fn init_matches_ui(matched_items: Vec<MatchItem>) {
  initscr();
  raw();
  noecho();

  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
  let (screen_width, screen_height) = get_screen_dimensions();

  refresh();

  let title_window = create_title_window(screen_width);
  let matches_window_height = screen_height - MATCHES_WINDOW_TOP_POS;
  let matches_window = create_matches_window(screen_width, matches_window_height);

  for (idx, match_item) in (&matched_items).iter().enumerate() {
    let full_str = format!(
      "{} [{}/{}]\n",
      match_item.path, match_item.num, match_item.total
    );

    if (idx as i32) > matches_window_height - 3 {
      break;
    }

    matches_window.print_line(&full_str, (idx + 1) as i32);
  }

  let mut ch = getch();

  while ch != KEY_Q {
    ch = getch();
  }

  destroy_window(title_window.orig_window);
  destroy_window(matches_window.orig_window);

  endwin();
}
