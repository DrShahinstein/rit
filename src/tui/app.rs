use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal};
use super::{ui, keys, git, git::file};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderChoice {
  MainMenu, CommitMenu,
}

pub struct App {
  render_choice: RenderChoice,
  branch:        String,
  changed_files: Vec<file::Changed>,
  last_error:    Option<String>,
  exit:          bool,
}

impl Default for App {
  fn default() -> Self {
    App {
      render_choice: RenderChoice::MainMenu,
      branch:        String::new(),
      changed_files: Vec::new(),
      last_error:    None,
      exit:          false,
    }
  }
}

impl App {
  pub fn new() -> Self {
    Self::default()
  }

  fn init(&mut self) {
    self.branch = match git::get_branch() {
      Ok(b)  => b,
      Err(e) => {
        self.last_error = Some(e.to_string());
        "unknown".to_string()
      },
    };

    self.changed_files = match git::get_changed_files() {
      Ok(v)  => v,
      Err(e) => {
        self.last_error = Some(e.to_string());
        Vec::new()
      },
    }
  }

  fn handle_events(&mut self) -> Result<()> {
    if let Event::Key(key) = event::read()? {
      if key.kind == KeyEventKind::Press {
        keys::handle_keys(self, key.code);
      }
    }
    Ok(())
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    self.init();

    while !self.exit {
      terminal.draw(|frame| ui::render(self, frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  pub fn shutdown(&mut self) {
    self.exit = true;
  }

  /* render_choice */
  pub fn get_render_choice(&self) -> RenderChoice { self.render_choice                             }
  pub fn go_main(&mut self)                       { self.render_choice = RenderChoice::MainMenu;   }
  pub fn go_commit(&mut self)                     { self.render_choice = RenderChoice::CommitMenu; }

  /* branch */
  pub fn get_branch(&self) -> &str      { &self.branch                 }
  pub fn set_branch(&mut self, b: &str) { self.branch = b.to_string(); }

  /* changed_files */
  pub fn get_changed_files(&self) -> &[file::Changed] { &self.changed_files }

  /* last_error */
  pub fn get_last_error(&self) -> &str {
    match &self.last_error {
      Some(msg) => msg,
      None      => "",
    }
  }
}
