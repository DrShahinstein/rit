use crossterm::event::KeyCode;
use super::app::App;

impl App {
  pub(super) fn handle_keys(&mut self, key: KeyCode) {
    match key {
      KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
      _ => {}
    }
  }
}
