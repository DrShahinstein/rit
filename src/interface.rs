use crate::app::App;
use crate::git::FileStatus;
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
    let layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Length(3), Constraint::Min(0)])
      .split(f.area());

    let branch_text = format!("branch: {}", self.current_branch);
    let header = Paragraph::new(Text::from(branch_text).fg(Color::Green))
      .block(Block::default().borders(Borders::BOTTOM))
      .alignment(Alignment::Center);
    f.render_widget(header, layout[0]);

    let items: Vec<ListItem> = self
      .changed_files
      .iter()
      .enumerate()
      .map(|(i, file_info)| {
        let prefix = if self.staged_indices.contains(&i) {
          Span::styled("[x] ", Style::default().fg(Color::LightGreen))
        } else {
          Span::styled("[ ] ", Style::default().fg(Color::White))
        };

        let status_span = match file_info.status {
          FileStatus::Untracked => Span::styled("(NEW)", Style::default().fg(Color::White).bold()),
          FileStatus::Modified  => Span::styled("M", Style::default().fg(Color::Green)),
          FileStatus::Deleted   => Span::styled("D", Style::default().fg(Color::Red)),
          FileStatus::Added     => Span::styled("A", Style::default().fg(Color::Green)),
          FileStatus::Renamed   => Span::styled("R", Style::default().fg(Color::Yellow)),
          _ => Span::raw(""),
        };

        let path_span = Span::raw(file_info.path.trim());
        let line = Line::from(vec![prefix, path_span, Span::raw(" "), status_span]);

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

    f.render_stateful_widget(changes_list, layout[1], &mut self.main_menu_state);
  }

  fn render_commit_menu(&mut self, f: &mut Frame) {
    // codecodecodecodecode
  }
}
