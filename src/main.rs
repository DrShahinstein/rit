use color_eyre::Result;
use rit_tui::tui::app::App;

fn main() -> Result<()> {
  color_eyre::install()?;

  let mut terminal = ratatui::init();
  let mut app      = App::new();
  let result       = app.run(&mut terminal);

  ratatui::restore();
  return result;
}
