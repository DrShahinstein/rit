use crate::{app::App, git::run_git};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  DefaultTerminal, Frame,
  buffer::Buffer,
  layout::Rect,
  style::Stylize,
  symbols::border,
  text::{Line, Text},
  widgets::{Block, Borders, Paragraph, Widget},
};
use std::io;

pub trait Renderable {
  fn render_main_menu(&self, f: &mut Frame);
  fn render_commit_menu(&self, f: &mut Frame);
}

impl Renderable for App {
  fn render_main_menu(&self, f: &mut Frame) {
    // codecodecodecodecode
  }

  fn render_commit_menu(&self, f: &mut Frame) {
    // codecodecodecodecode
  }
}
