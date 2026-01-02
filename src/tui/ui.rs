#[allow(unused_imports)]
use ratatui::{prelude::{*}, widgets::{*}, style::{*}};
use super::{app::App, app::RenderChoice, git::file};

pub fn render(app: &mut App, frame: &mut Frame) {
  let err = app.get_last_error();
  if !err.is_empty() {
    help::render_error(err, frame);
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
    .map(|f: &file::Changed| {
      let checked  = f.is_staged();
      let checkbox = help::checkbox(checked);
      let path     = f.path.clone();
      let worktree = f.worktree.as_char();
      let index    = f.index.as_char();

      let wt  = if index    == ' ' {'*'} else {index};
      let idx = if worktree == ' ' {'*'} else {worktree};

      let line = Line::from(vec![
        Span::styled(checkbox, help::checkbox_color(checked)),
        Span::raw(" "),
        Span::raw(path),
        Span::raw(" "),
        Span::styled(wt.to_string(),  help::colored(wt)),
        Span::styled(idx.to_string(), help::colored(idx)),
      ]);

      ListItem::new(line)
    })
    .collect();

  let list = List::new(files)
    .block(Block::default().borders(Borders::ALL).title("Changes"))
    .highlight_style(
      Style::default()
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

  frame.render_stateful_widget(list, area, app.get_listview_mut());
}

fn commit_menu(_app: &mut App, frame: &mut Frame, area: Rect) {
  frame.render_widget("Commit", area);
}


mod help {
  #[allow(unused_imports)]
  use ratatui::{prelude::{*}, widgets::{*}, style::{*}};

  pub fn render_error(msg: &str, frame: &mut Frame) {
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

  pub fn checkbox(yes: bool) -> &'static str {
    if yes {"[x]"} else {"[ ]"}
  }

  pub fn checkbox_color(yes: bool) -> Style {
    let s = Style::default();
    if yes { s.fg(Color::LightGreen) } else { s.fg(Color::White) }
  }

  pub fn colored(c: char) -> Style {
    let style = Style::default().add_modifier(Modifier::BOLD);

    match c {
      'M' => style.fg(Color::Green),
      'D' => style.fg(Color::Red),
      'A' => style.fg(Color::Green),
      'R' => style.fg(Color::Yellow),
      'C' => style.fg(Color::Yellow),
      'U' => style.fg(Color::Magenta),
      '?' => style.fg(Color::White),
      ' ' => Style::default().fg(Color::DarkGray),
      _   => Style::default(),
    }
  }
}
