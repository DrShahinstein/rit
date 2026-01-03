use ratatui::crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use super::app::{App, RenderChoice};

pub fn handle_keys(app: &mut App, key: KeyEvent) {
  match app.get_render_choice() {
    RenderChoice::MainMenu   => main_menu_keys(app, key),
    RenderChoice::CommitMenu => commit_menu_keys(app, key),
  }
}

fn main_menu_keys(app: &mut App, key: KeyEvent) {
  match key.code {
    KeyCode::Char('q') => { app.shutdown(); return; },
    KeyCode::Char('c') =>   app.go_commit(),
    KeyCode::Char('r') =>   app.refresh(),
    KeyCode::Down      =>   app.select_next(),
    KeyCode::Up        =>   app.select_prev(),
    KeyCode::Enter     =>   app.toggle_file_stage(),
    _ => {}
  }
}

fn commit_menu_keys(app: &mut App, key: KeyEvent) {
  match key.code {
    KeyCode::Esc => { app.go_main(); return; },

    /* commit on ctrl+s */
    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
      app.commit();
      return;
    }
    _ => {}
  }

  app.get_textarea_mut().input(key);
}
