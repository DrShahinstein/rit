use rit::app::App;
use std::io;

mod ratatui_helpers {
  use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
  };
  use ratatui::{Terminal, prelude::*};
  use std::io::{self, stdout};

  pub type DefaultTerminal = Terminal<CrosstermBackend<io::Stdout>>;

  pub fn init() -> io::Result<DefaultTerminal> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
  }

  pub fn restore() {
    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = disable_raw_mode();
  }
}

fn main() -> io::Result<()> {
  let mut terminal = ratatui_helpers::init()?;
  let app_result = App::new().run(&mut terminal);
  ratatui_helpers::restore();
  app_result
}
