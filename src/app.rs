use crate::{git, interface::Renderable};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{Frame, prelude::*, widgets::ListState};
use std::{collections::HashSet, io};

type DefaultTerminal = Terminal<CrosstermBackend<io::Stdout>>;

#[derive(Debug, Default)]
enum RenderChoice {
  #[default]
  MainMenu,
  CommitMenu,
}

#[derive(Debug)]
pub struct App {
  render: RenderChoice,
  pub changed_files: Vec<String>,
  pub staged_indices: HashSet<usize>,
  pub commit_msg: String,
  pub main_menu_state: ListState,
  pub current_branch: String,
  pub exit: bool,
}

impl Default for App {
  fn default() -> Self {
    App {
      render: RenderChoice::MainMenu,
      changed_files: Vec::new(),
      staged_indices: HashSet::new(),
      commit_msg: String::new(),
      main_menu_state: ListState::default(),
      current_branch: "none".to_string(),
      exit: false,
    }
  }
}

impl App {
  pub fn new() -> Self {
    let mut app = App {
      current_branch: git::get_current_branch().unwrap_or_else(|_| "detached".to_string()),
      changed_files:  git::get_changed_files().unwrap_or_default(),
      staged_indices: git::get_staged_indices().unwrap_or_default(),
      ..Default::default()
    };

    if !app.changed_files.is_empty() {
      app.main_menu_state.select(Some(0));
    }

    app
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.draw(frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  fn draw(&mut self, f: &mut Frame) {
    match self.render {
      RenderChoice::MainMenu   => self.render_main_menu(f),
      RenderChoice::CommitMenu => self.render_commit_menu(f),
    }
  }

  fn handle_events(&mut self) -> io::Result<()> {
    if let Event::Key(key_event) = event::read()? {
      if key_event.kind == KeyEventKind::Press {
        self.handle_key_event(key_event);
      }
    }
    Ok(())
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    match self.render {
      RenderChoice::MainMenu => self.handle_main_menu_keys(key_event),
      RenderChoice::CommitMenu => { /* self.handle_commit_menu_keys(key_event) */ }
    }
  }

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
      let file_path = &self.changed_files[selected_index];
      if self.staged_indices.contains(&selected_index) {
        let _ = git::run_git(&["reset", "HEAD", "--", file_path]);
        self.staged_indices.remove(&selected_index);
      } else {
        let _ = git::run_git(&["add", file_path]);
        self.staged_indices.insert(selected_index);
      }
    }
  }

  fn exit(&mut self) {
    self.exit = true;
  }
}
