use crate::app::App;
use crate::git::FileStatus;
use ratatui::{
  prelude::*,
  widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

pub trait Renderable {
  fn render_main_menu(&mut self, f: &mut Frame);
  fn render_commit_menu(&mut self, f: &mut Frame);
}

impl Renderable for App {
  fn render_main_menu(&mut self, f: &mut Frame) {
    let layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Length(1), Constraint::Min(0)])
      .split(f.area());

    let branch_text = format!("branch: {}", self.current_branch);
    let header = Paragraph::new(Text::from(branch_text).fg(Color::Green)).alignment(Alignment::Center);
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
    self.render_main_menu(f);

    let popup_area = centered_rect(60, 20, f.area());
    let commit_input = Paragraph::new(self.commit_msg.as_str())
      .style(Style::default().fg(Color::White))
      .wrap(Wrap { trim: false })
      .block(
        Block::default()
          .borders(Borders::ALL)
          .title("Commit Message")
          .border_style(Style::default().fg(Color::LightBlue)),
      );

    f.render_widget(Clear, popup_area);
    f.render_widget(commit_input, popup_area);
    f.set_cursor_position(Position::new(
      popup_area.x + 1 + self.cursor_pos as u16,
      popup_area.y + 1,
    ));
  }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
