use crate::app::App;
use ratatui::{
  prelude::*,
  widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub trait Renderable {
  fn render_main_menu(&mut self, f: &mut Frame);
  fn render_commit_menu(&mut self, f: &mut Frame);
}

impl Renderable for App {
  fn render_main_menu(&mut self, f: &mut Frame) {
    let main_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Length(3), Constraint::Min(0)])
      .split(f.area());

    let branch_text = format!("branch: {}", self.current_branch);
    let header = Paragraph::new(Text::from(branch_text).fg(Color::Green))
      .block(Block::default().borders(Borders::BOTTOM))
      .alignment(Alignment::Center);
    f.render_widget(header, main_layout[0]);

    let items: Vec<ListItem> = self
      .changed_files
      .iter()
      .enumerate()
      .map(|(i, file)| {
        let prefix = if self.staged_indices.contains(&i) {
          Span::styled("[x] ", Style::default().fg(Color::LightGreen))
        } else {
          Span::styled("[ ] ", Style::default().fg(Color::White))
        };
        let line = Line::from(vec![prefix, Span::raw(file.trim())]);
        ListItem::new(line)
      })
      .collect();

    let changes_list = List::new(items)
      .block(Block::default().borders(Borders::ALL).title("Changes"))
      .highlight_style(
        Style::default()
          .bg(Color::DarkGray)
          .add_modifier(Modifier::BOLD),
      )
      .highlight_symbol(">> ");

    f.render_stateful_widget(changes_list, main_layout[1], &mut self.main_menu_state);
  }

  fn render_commit_menu(&mut self, f: &mut Frame) {
    // codecodecodecodecode
  }
}
