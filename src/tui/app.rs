use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use super::{ui, keys};

pub enum RenderChoice {
  MainMenu, CommitMenu,
}

pub struct App {
  pub render_choice: RenderChoice,
  exit:              bool,
}

impl App {
  pub fn new() -> Self {
    Self {
      render_choice: RenderChoice::MainMenu,
      exit:          false,
    }
  }

  pub fn shutdown(&mut self) {
    self.exit = true;
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.draw(frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  fn draw(&self, frame: &mut Frame) {
    ui::draw(self, frame);
  }

  fn handle_events(&mut self) -> Result<()> {
    if let Event::Key(key) = event::read()? {
      if key.kind == KeyEventKind::Press {
        keys::handle_keys(self, key.code);
      }
    }
    Ok(())
  }
}
