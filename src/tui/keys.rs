use crossterm::event::KeyCode;
use super::app::App;

pub fn handle_keys(app: &mut App, key: KeyCode) {
  match key {
    KeyCode::Char('q') | KeyCode::Esc => app.shutdown(),
    _ => {}
  }
}
