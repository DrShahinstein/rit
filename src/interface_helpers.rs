use ratatui::prelude::*;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  let layout = Layout::default()
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
    .split(layout[1])[1]
}

pub fn make_cursor_aiming(f: &mut Frame, area: Rect, cursor_pos: usize, commit_msg: &str) {
  let width = area.width.saturating_sub(2) as usize;   // usable cols
  let height = area.height.saturating_sub(2) as usize; // usable rows
  let before_cursor = &commit_msg[..cursor_pos];
  let mut v_row = 0;
  let mut v_col = 0;

  for c in before_cursor.chars() {
    match c {
      '\n' => {
        v_row += 1;
        v_col = 0;
      }
      _ => {
        if v_col >= width {
          v_row += 1;
          v_col = 0;
        }
        v_col += 1;
      }
    }
  }

  let visual_row = v_row.min(height.saturating_sub(1));
  let visual_col = v_col.min(width.saturating_sub(1));

  f.set_cursor_position(Position::new(
    area.x + 1 + visual_col as u16,
    area.y + 1 + visual_row as u16,
  ));
}
