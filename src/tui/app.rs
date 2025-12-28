use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use super::{ui, keys, git};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderChoice {
  MainMenu, CommitMenu,
}

pub struct App {
  render_choice: RenderChoice,
  branch:        String,
  exit:          bool,
}

impl Default for App {
  fn default() -> Self {
    App {
      render_choice: RenderChoice::MainMenu,
      branch:        "unknown".to_string(),
      exit:          false,
    }
  }
}

impl App {
  pub fn new() -> Self {
    Self {
      branch: git::get_branch().unwrap_or_default(), 
      ..Default::default()
    }
  }

  pub fn shutdown(&mut self) {
    self.exit = true;
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    while !self.exit {
      terminal.draw(|frame| ui::render(self, frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  fn handle_events(&mut self) -> Result<()> {
    if let Event::Key(key) = event::read()? {
      if key.kind == KeyEventKind::Press {
        keys::handle_keys(self, key.code);
      }
    }
    Ok(())
  }

  /* render_choice */
  pub fn get_render_choice(&self) -> RenderChoice { return self.render_choice;                     }
  pub fn go_main(&mut self)                       { self.render_choice = RenderChoice::MainMenu;   }
  pub fn go_commit(&mut self)                     { self.render_choice = RenderChoice::CommitMenu; }

  /* branch */
  pub fn get_branch(&self) -> String    { return self.branch.clone(); }
  pub fn set_branch(&mut self, b: &str) { self.branch = b.to_string(); }
}
