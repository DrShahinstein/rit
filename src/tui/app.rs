use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

pub struct App {
  pub exit: bool,
}

impl App {
  pub fn new() -> Self {
    Self {
      exit: false,
    }
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.draw(frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  fn draw(&self, frame: &mut Frame) {
    frame.render_widget("Hello World", frame.area());
  }

  fn handle_events(&mut self) -> Result<()> {
    if let Event::Key(key) = event::read()? {
      if key.kind == KeyEventKind::Press {
        self.handle_keys(key.code); // keys.rs
      }
    }
    Ok(())
  }
}
