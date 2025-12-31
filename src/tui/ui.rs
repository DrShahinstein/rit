#[allow(unused_imports)]
use ratatui::{prelude::{*}, widgets::{*}};
use super::app::{App, RenderChoice};

pub fn render(app: &mut App, frame: &mut Frame) {
  let err = app.get_last_error();
  if !err.is_empty() {
    render_error(err, frame);
    return;
  }

  let layout =  Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
    .split(frame.area());

  let branch = format!("branch: {}", app.get_branch());
  let header = Paragraph::new(branch)
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Green));

  frame.render_widget(header, layout[0]);

  match app.get_render_choice() {
    RenderChoice::MainMenu   => main_menu(app,   frame, layout[1]),
    RenderChoice::CommitMenu => commit_menu(app, frame, layout[1]),
  }

  let footer = Paragraph::new("q: quit    c: commit    r: refresh")
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::LightBlue));

  frame.render_widget(footer, layout[2]);
}

fn main_menu(app: &mut App, frame: &mut Frame, area: Rect) {
  let files: Vec<ListItem> = app
    .get_changed_files()
    .iter()
    .map(|f| {
      let line = format!("{}{} {}", f.index.as_char(), f.worktree.as_char(), f.path);
      ListItem::new(line)
    })
    .collect();

  let list = List::new(files)
    .block(Block::default().borders(Borders::ALL).title("Changes"))
    .highlight_style(
      Style::default()
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),)
    .highlight_symbol(">> ");

  frame.render_stateful_widget(list, area, app.get_listview_mut());
}

fn commit_menu(_app: &mut App, frame: &mut Frame, area: Rect) {
  frame.render_widget("Commit", area);
}

fn render_error(msg: &str, frame: &mut Frame) {
  let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(0), Constraint::Length(1),
    ])
    .split(frame.area());

  let block = Block::default()
    .borders(Borders::ALL)
    .title("error");

  let text = Paragraph::new(msg.to_string())
    .block(block)
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true })
    .style(Style::default().fg(Color::Red));

  let footer = Paragraph::new("press q to quit")
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::LightBlue));

  frame.render_widget(text,   layout[0]);
  frame.render_widget(footer, layout[1]);
}
