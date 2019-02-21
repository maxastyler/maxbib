use crate::query::QueryData;
use std::sync::mpsc;
use std::thread;

use termion::clear;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List, Paragraph, SelectableList, Text, Widget};
use tui::Terminal;

pub struct App {
    data: Vec<QueryData>,
    selected: Option<usize>,
    search: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            data: vec![],
            selected: None,
            search: vec![],
        }
    }
}

impl From<Vec<QueryData>> for App {
    fn from(data: Vec<QueryData>) -> Self {
        let length = data.get(0).and_then(|x| Some(x.len())).unwrap_or(0);
        App {
            data: data,
            selected: None,
            search: vec![String::new(); length],
        }
    }
}

impl App {
    pub fn run(&mut self) -> std::io::Result<()> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let stdout = MouseTerminal::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        loop {
            terminal.draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.size());
                let list_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                    .split(chunks[0]);
                SelectableList::default().items(&vec!["hi", "there", "how", "are", "you"]).render(&mut f, list_chunks[0]);
            })?;
        }
        Ok(())
    }
}
