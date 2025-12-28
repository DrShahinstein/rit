#[allow(unused_imports)]
use ratatui::{prelude::{*}, widgets::{*}};
use super::app::{App, RenderChoice};

pub fn render(app: &App, frame: &mut Frame) {
  let chunks =  Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(1), Constraint::Min(0)])
    .split(frame.area());

  let branch = format!("branch: {}", app.get_branch());
  let style  = Style::default().fg(Color::Green);

  let header = Paragraph::new(branch)
    .alignment(Alignment::Center)
    .style(style);

  frame.render_widget(header, chunks[0]);

  match app.get_render_choice() {
    RenderChoice::MainMenu => {
      frame.render_widget("Main", chunks[1]);
    }
    RenderChoice::CommitMenu => {
      frame.render_widget("Commit", chunks[1]);
    }
  }
}
