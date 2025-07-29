use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  buffer::Buffer,
  layout::Rect,
  style::Stylize,
  symbols::border,
  text::{Line, Text},
  widgets::{Block, Paragraph, Widget},
};
use std::io;

use crate::git::run_git;

#[derive(Debug, Default)]
pub struct App {
  git_adds: Vec<String>,
  git_commit: String,
  exit: bool,
}

impl App {
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.draw(frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  fn draw(&self, f: &mut Frame) {
    f.render_widget(self, f.area());
  }

  fn handle_events(&mut self) -> io::Result<()> {
    match event::read()? {
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        self.handle_key_event(key_event)
      }
      _ => {}
    };
    Ok(())
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Char('q') => self.exit(),
      _ => {}
    }
  }

  fn exit(&mut self) {
    self.exit = true;
  }
}

impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let status = run_git("status").unwrap_or_else(|e| e.to_string());
    Paragraph::new(status).centered().render(area, buf);
  }
}
