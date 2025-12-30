#[allow(unused_imports)]
use ratatui::{prelude::{*}, widgets::{*}};
use super::app::{App, RenderChoice};

pub fn render(app: &App, frame: &mut Frame) {
  let chunks =  Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(1), Constraint::Min(0)])
    .split(frame.area());

  let branch = format!("branch: {}", app.get_branch());
  let header = Paragraph::new(branch)
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Green));

  frame.render_widget(header, chunks[0]);

  match app.get_render_choice() {
    RenderChoice::MainMenu   => main_menu(&app,   frame, chunks[1]),
    RenderChoice::CommitMenu => commit_menu(&app, frame, chunks[1]),
  }
}

fn main_menu(app: &App, frame: &mut Frame, area: Rect) {
  let files: Vec<ListItem> = app
    .get_changed_files()
    .iter()
    .map(|f| {
                       // XY <path>
      let line = format!("{}{} {}", f.index.as_char(), f.worktree.as_char(), f.path);
      ListItem::new(line)
    })
    .collect();

  let list = List::new(files)
    .block(Block::default().title("Changes").borders(Borders::ALL));

  frame.render_widget(list, area);
}

fn commit_menu(_app: &App, frame: &mut Frame, area: Rect) {
  frame.render_widget("Commit", area);
}
