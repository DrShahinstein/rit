use ratatui::{Frame, widgets::{*}, layout::{*}};
use super::app::App;

pub fn draw(app: &App, frame: &mut Frame) {
  frame.render_widget("Hello World", frame.area());
}
