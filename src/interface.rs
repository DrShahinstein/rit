use crate::app::App;
use crate::git::FileStatus;
use crate::interface_helpers;
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
      .constraints([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
      ])
      .split(f.area());

    let branch_text = format!("branch: {}", self.current_branch);
    let color = if self.current_branch == "detached" {Color::Red} else {Color::Green};
    let header = Paragraph::new(Text::from(branch_text).fg(color)).alignment(Alignment::Center);
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

    let footer = Paragraph::new("q: quit    c: commit")
      .alignment(Alignment::Center)
      .style(Style::default().fg(Color::LightBlue));
    f.render_widget(footer, layout[2]);
  }

  fn render_commit_menu(&mut self, f: &mut Frame) {
    self.render_main_menu(f);

    let popup_area = interface_helpers::centered_rect(60, 20, f.area());
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
    interface_helpers::make_cursor_aiming(f, popup_area, self.cursor_pos, &self.commit_msg);
  }
}
