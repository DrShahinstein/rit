use crate::app::{App, RenderChoice};
use crate::git;
use crossterm::event::{KeyCode, KeyEvent};

pub trait KeysInteractable {
  fn handle_main_menu_keys(&mut self, key_event: KeyEvent);
  fn handle_commit_menu_keys(&mut self, key_event: KeyEvent);
  fn select_next(&mut self);
  fn select_previous(&mut self);
  fn toggle_stage_selection(&mut self);
}

impl KeysInteractable for App {
  fn handle_main_menu_keys(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Char('q') => self.exit(),
      KeyCode::Char('c') if !self.staged_indices.is_empty() => {
        self.render = RenderChoice::CommitMenu;
      }
      KeyCode::Down | KeyCode::Char('j') => self.select_next(),
      KeyCode::Up | KeyCode::Char('k') => self.select_previous(),
      KeyCode::Enter => self.toggle_stage_selection(),
      _ => {}
    }
  }

  fn handle_commit_menu_keys(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Char(c) => {
        self.commit_msg.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
      }
      KeyCode::Backspace => {
        if self.cursor_pos > 0 {
          self.cursor_pos -= 1;
          self.commit_msg.remove(self.cursor_pos);
        }
      }
      KeyCode::Left => {
        if self.cursor_pos > 0 {
          self.cursor_pos -= 1;
        }
      }
      KeyCode::Right => {
        if self.cursor_pos < self.commit_msg.len() {
          self.cursor_pos += 1;
        }
      }
      KeyCode::Enter => {
        if !self.commit_msg.is_empty() {
          self.commit_changes();
        }
      }
      KeyCode::Esc => {
        self.render = RenderChoice::MainMenu;
        self.commit_msg.clear();
        self.cursor_pos = 0;
      }
      _ => {}
    }
  }

  fn select_next(&mut self) {
    if self.changed_files.is_empty() {
      return;
    }
    let i = self.main_menu_state.selected().map_or(0, |i| {
      if i >= self.changed_files.len() - 1 {
        0
      } else {
        i + 1
      }
    });
    self.main_menu_state.select(Some(i));
  }

  fn select_previous(&mut self) {
    if self.changed_files.is_empty() {
      return;
    }
    let i = self.main_menu_state.selected().map_or(0, |i| {
      if i == 0 {
        self.changed_files.len() - 1
      } else {
        i - 1
      }
    });
    self.main_menu_state.select(Some(i));
  }

  fn toggle_stage_selection(&mut self) {
    if let Some(selected_index) = self.main_menu_state.selected() {
      let file_path = &self.changed_files[selected_index].path;
      if self.staged_indices.contains(&selected_index) {
        let _ = git::run_git(&["reset", "HEAD", "--", file_path]);
        self.staged_indices.remove(&selected_index);
      } else {
        let _ = git::run_git(&["add", file_path]);
        self.staged_indices.insert(selected_index);
      }
    }
  }
}
