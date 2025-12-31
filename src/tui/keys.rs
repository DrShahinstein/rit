use crossterm::event::KeyCode;
use super::app::{App, RenderChoice};

pub fn handle_keys(app: &mut App, key: KeyCode) {
  match key {
    KeyCode::Char('q') => { app.shutdown(); return; },
    _ => {}
  }

  match app.get_render_choice() {
    RenderChoice::MainMenu   => main_menu_keys(app, key),
    RenderChoice::CommitMenu => commit_menu_keys(app, key),
  }
}

fn main_menu_keys(app: &mut App, key: KeyCode) {
  match key {
    KeyCode::Char('c') => app.go_commit(),
    KeyCode::Down      => app.select_next(),
    KeyCode::Up        => app.select_prev(),
    _ => {}
  }
}

fn commit_menu_keys(app: &mut App, key: KeyCode) {
  match key {
    KeyCode::Esc => app.go_main(),
    _ => {}
  }
}
