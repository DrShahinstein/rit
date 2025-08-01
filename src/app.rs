use crate::keys::KeysInteractable;
use crate::{git, interface::Renderable};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{Frame, prelude::*, widgets::ListState};
use std::{collections::HashSet, io};

type DefaultTerminal = Terminal<CrosstermBackend<io::Stdout>>;

#[derive(Debug, Default)]
pub enum RenderChoice {
  #[default]
  MainMenu,
  CommitMenu,
}

#[derive(Debug)]
pub struct App {
  pub render: RenderChoice,
  pub changed_files: Vec<git::ChangedFile>,
  pub staged_indices: HashSet<usize>,
  pub commit_msg: String,
  pub cursor_pos: usize,
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
      cursor_pos: 0,
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
      RenderChoice::CommitMenu => self.handle_commit_menu_keys(key_event),
    }
  }

  pub fn commit_changes(&mut self) {
    if git::commit(&self.commit_msg).is_ok() {
      self.changed_files = git::get_changed_files().unwrap_or_default();
      self.staged_indices = git::get_staged_indices().unwrap_or_default();
      self.commit_msg.clear();
      self.cursor_pos = 0;
      self.render = RenderChoice::MainMenu;
      self.main_menu_state.select(Some(0));
    } // else
  }

  pub fn exit(&mut self) {
    self.exit = true;
  }
}
