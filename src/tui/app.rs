use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

pub struct App {
  message: String,
  exit: bool,
}

impl App {
  pub fn new() -> Self {
    Self {
      message: String::from("HELLOEĞEĞEĞEĞEE"),
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
    frame.render_widget(self.message.as_str(), frame.area());
  }

  fn handle_events(&mut self) -> Result<()> {
    if let Event::Key(key) = event::read()? {
      if key.kind == KeyEventKind::Press {
        match key.code {
          KeyCode::Char('q') | KeyCode::Esc => self.exit = true, _ => {}
        }
      }
    }
    Ok(())
  }
}
