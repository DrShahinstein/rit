use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::ListState};
use super::{ui, keys, git, git::file};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderChoice {
  MainMenu, CommitMenu,
}

pub struct App {
  render_choice: RenderChoice,
  branch:        String,
  changed_files: Vec<file::Changed>,
  listview:      ListState,
  last_error:    Option<String>,
  exit:          bool,
}

impl Default for App {
  fn default() -> Self {
    App {
      render_choice: RenderChoice::MainMenu,
      branch:        String::new(),
      changed_files: Vec::new(),
      listview:      ListState::default(),
      last_error:    None,
      exit:          false,
    }
  }
}

impl App {
  pub fn new() -> Self {
    Self::default()
  }

  fn init(&mut self) {
    self.branch = match git::get_branch() {
      Ok(b)  => b,
      Err(e) => {
        self.last_error = Some(e.to_string());
        "unknown".to_string()
      },
    };

    self.changed_files = match git::get_changed_files() {
      Ok(v)  => v,
      Err(e) => {
        self.last_error = Some(e.to_string());
        Vec::new()
      },
    };
    
    if self.changed_files.is_empty() {
      self.listview.select(None);
    } else if self.listview.selected().is_none() {
      self.listview.select(Some(0));
    } else {
      let len = self.changed_files.len();

      let already = match len {
        0 => None,
        _ => Some(self.listview.selected().unwrap_or(0).min(len - 1)),
      };

      self.listview.select(already);
    }
  }

  fn handle_events(&mut self) -> Result<()> {
    if let Event::Key(key) = event::read()? {
      if key.kind == KeyEventKind::Press {
        keys::handle_keys(self, key.code);
      }
    }
    Ok(())
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    self.refresh();

    while !self.exit {
      terminal.draw(|frame| ui::render(self, frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  pub fn shutdown(&mut self) { self.exit = true; }
  pub fn refresh(&mut self)  { self.init();      }
 
  pub fn toggle_file_stage(&mut self) {
    let Some(i) = self.listview.selected()  else { return; };
    let Some(f) = self.changed_files.get(i) else { return; };

    let path   = &f.path;
    let staged = f.is_staged();

    let res = if staged {
      git::unstage(path)
    } else {
      git::stage(path)
    };

    if let Err(e) = res {
      self.last_error = Some(e.to_string());
      return;
    }

    self.refresh();
  }

  /* render_choice */
  pub fn get_render_choice(&self) -> RenderChoice { self.render_choice                             }
  pub fn go_main(&mut self)                       { self.render_choice = RenderChoice::MainMenu;   }
  pub fn go_commit(&mut self)                     { self.render_choice = RenderChoice::CommitMenu; }

  /* branch */
  pub fn get_branch(&self) -> &str      { &self.branch                 }
  pub fn set_branch(&mut self, b: &str) { self.branch = b.to_string(); }

  /* changed_files */
  pub fn get_changed_files(&self) -> &[file::Changed] { &self.changed_files }

  /* listview */
  pub fn get_listview(&self)         -> &ListState     { &self.listview     }
  pub fn get_listview_mut(&mut self) -> &mut ListState { &mut self.listview }
  pub fn select_next(&mut self) {
    let len = self.changed_files.len();
    if len == 0 {
      self.listview.select(None);
      return;
    }

    let i    = self.listview.selected().unwrap_or(0);
    let next = if i + 1 >= len {0} else {i+1};

    self.listview.select(Some(next));
  }
  pub fn select_prev(&mut self) {
    let len = self.changed_files.len();
    if len == 0 {
      self.listview.select(None);
      return;
    }

    let i    = self.listview.selected().unwrap_or(0);
    let prev = if i == 0 {len-1} else {i-1};

    self.listview.select(Some(prev));
  }

  /* last_error */
  pub fn get_last_error(&self) -> &str {
    match &self.last_error {
      Some(msg) => msg,
      None      => "",
    }
  }
}
