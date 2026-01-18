#[allow(unused_imports)]
use ratatui::{prelude::{*}, widgets::{*}, style::{*}};
use super::{app::App, app::RenderChoice, app::SelectionMode,git::file};

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

  let help_keys = help::keys_for(app.get_render_choice());
  let footer    = Paragraph::new(help_keys)
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::LightBlue));

  frame.render_widget(footer, layout[2]);
}

fn main_menu(app: &mut App, frame: &mut Frame, area: Rect) {
  let selection_mode = app.get_selection_mode();
  let avail_width    = area.width.saturating_sub(6) as usize; // 2 for borders, 4 for ">> "

  let files: Vec<ListItem> = app
    .get_changed_files()
    .iter()
    .map(|f: &file::Changed| {
      let checked      = f.is_staged();
      let checkbox     = help::checkbox(checked);
      let path         = f.path.clone();
      let mut index    = f.index.as_char();
      let mut worktree = f.worktree.as_char();

      if index    == ' ' { index='*';    };
      if worktree == ' ' { worktree='*'; };

      let (stage_style, discard_style, discard_symbol) = match selection_mode {
        SelectionMode::Stage => (
          help::checkbox_color(checked),
          Style::default().fg(Color::Yellow),
          "↻",
        ),
        SelectionMode::Discard => (
          Style::default().fg(Color::DarkGray),
          Style::default()
            .fg(Color::LightRed)
            .add_modifier(Modifier::BOLD),
          "[ ↻ ]",
        ),
      };

      let content_width = checkbox.len() + 1 + path.len() + 1 + 2;
      let padding =
      /* to push discard symbol to the right */
      if content_width + discard_symbol.len() + 1 < avail_width {
        avail_width.saturating_sub(content_width + discard_symbol.len() - 1)
      } else {
        1 // min spacing
      };

      let line = Line::from(vec![
        Span::styled(checkbox, stage_style),
        Span::raw(" "),
        Span::raw(path),
        Span::raw(" "),
        Span::styled(index.to_string(),    help::colored(index)),
        Span::styled(worktree.to_string(), help::colored(worktree)),
        Span::raw(" ".repeat(padding)),
        Span::styled(discard_symbol, discard_style),
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

fn commit_menu(app: &mut App, frame: &mut Frame, area: Rect) {
  main_menu(app, frame, area);

  let popup = help::centered_rect(60, 20, frame.area());
  frame.render_widget(Clear, popup);

  let textarea = app.get_textarea_mut();
  help::customize_textarea(textarea);

  frame.render_widget(&*textarea, popup);
}


mod help {
  #[allow(unused_imports)]
  use ratatui::{prelude::{*}, widgets::{*}, style::{*}};
  use super::RenderChoice;
  use tui_textarea::TextArea;

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

  pub fn keys_for(choice: RenderChoice) -> &'static str {
    match choice {
      RenderChoice::MainMenu   => "q: quit    c: commit    ←/→: stage/discard    r: refresh",
      RenderChoice::CommitMenu => "esc: back   commit: ctrl+s   next-line: enter",
    }
  }

  pub fn customize_textarea(t: &mut TextArea) {
    let border   = Style::default().fg(Color::LightMagenta);

    t.set_style(Style::default().fg(Color::White));
    t.set_cursor_line_style(Style::default());
    t.set_block(
      Block::default()
        .borders(Borders::ALL)
        .border_style(border)
        .title(Span::styled("Commit Message", border.add_modifier(Modifier::BOLD))),
    );
  }

  /* for textarea */
  pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vert = Layout::default()
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
      .split(vert[1])[1]
  }
}
