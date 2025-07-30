use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;

use crate::interface::Renderable;

#[derive(Debug, Default)]
enum RenderChoice {
  #[default]
  MainMenu,
  CommitMenu,
}

#[derive(Debug, Default)]
pub struct App {
  render: RenderChoice,
  pub staged_files: Vec<String>,
  pub commit_msg: String,
  pub selected_index: u8,
  pub exit: bool,
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
    match self.render {
      RenderChoice::MainMenu => {
        self.render_main_menu(f);
      }
      RenderChoice::CommitMenu => {
        self.render_commit_menu(f);
      }
    }
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
      KeyCode::Char('c') => {
        self.render = RenderChoice::CommitMenu;
      }
      _ => {}
    }
  }

  fn exit(&mut self) {
    self.exit = true;
  }
}
