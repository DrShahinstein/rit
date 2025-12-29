use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal};
use super::{ui, keys, git};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderChoice {
  MainMenu, CommitMenu,
}

pub struct App {
  render_choice: RenderChoice,
  branch:        String,
  last_error:    Option<String>,
  exit:          bool,
}

impl Default for App {
  fn default() -> Self {
    App {
      render_choice: RenderChoice::MainMenu,
      branch:        String::new(),
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
  pub fn get_branch(&self) -> String    { self.branch.clone()          }
  pub fn set_branch(&mut self, b: &str) { self.branch = b.to_string(); }

  /* last_error */
  pub fn get_last_error(&self) -> String {
    match &self.last_error {
      Some(msg) => msg.to_string(),
      None      => String::new(),
    }
  }
}
