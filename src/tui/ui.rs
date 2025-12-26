#[allow(unused_imports)]
use ratatui::{Frame, widgets::{*}, layout::{*}};
use super::app::{App, RenderChoice};

pub fn draw(app: &App, frame: &mut Frame) {
  match app.get_render_choice() {
    RenderChoice::MainMenu => {
      frame.render_widget("Main", frame.area());
    }
    RenderChoice::CommitMenu => {
      frame.render_widget("Commit", frame.area());
    }
  }
}
