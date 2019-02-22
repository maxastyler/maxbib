use std::sync::mpsc;
use std::thread;

use termion::clear;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List, Paragraph, SelectableList, Text, Widget};
use tui::Terminal;

use crate::event::{Event, Events};
use crate::query::QueryData;
use crate::search::rank_query;

pub struct App {
    data: Vec<QueryData>,
    selected: Option<usize>,
    search: Vec<String>,
    search_number: usize,
}

impl Default for App {
    fn default() -> Self {
        App {
            data: vec![],
            selected: None,
            search: vec![],
            search_number: 0,
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
            search_number: 0,
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

        let events = Events::new();
        let (tx, rx) = mpsc::channel();
        let d = self.data.clone();
        thread::spawn(move || {
            let mut ranks: Vec<_> = d
                .into_iter()
                .map(|x| {
                    let r = rank_query(&x, &vec!["entangle", "a", "2"]);
                    (x, r)
                })
                .filter(|(_, r)| !r.is_nan())
                .collect();
            ranks.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
            tx.send(ranks);
        });
        let ranked_stuff = rx.recv().unwrap();

        terminal.hide_cursor()?;
        let mut selected = 0;
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
                SelectableList::default()
                    .items(
                        &ranked_stuff
                            .iter()
                            .map(|(x, _)| x.clone().strings[0].clone())
                            .collect::<Vec<String>>(),
                    )
                    .select(Some(selected))
                    .highlight_symbol(">")
                    .render(&mut f, list_chunks[0]);

                Paragraph::new(vec![Text::raw(format!("{}", selected))].iter())
                    .render(&mut f, list_chunks[1]);
            })?;

            match events.next().unwrap() {
                Event::Input(input) => match input {
                    Key::Ctrl('c') => {
                        break;
                    }
                    Key::Ctrl('l') | Key::Down => {
                        selected += 1;
                        if selected >= ranked_stuff.len() {
                            selected = 0;
                        }
                    }
                    Key::Ctrl('k') | Key::Up => {
                        if selected == 0 {
                            selected = ranked_stuff.len() - 1;
                        } else {
                            selected -= 1
                        }
                    },
                    Key::Backspace => {
                        self.search[0].pop();
                    },
                    _ => {}
                },
                _ => (),
            }
        }
        Ok(())
    }

    pub fn run_query(&self, tx: mpsc::Sender<(usize, Vec<(QueryData, f64)>)>) {
        let data = self.data.clone();
        let current_number = self.search_number;
        thread::spawn(move || {
            let mut ranks: Vec<_> = data
                .into_iter()
                .map(|x| {
                    let r = rank_query(&x, &vec!["entangle", "a", "2"]);
                    (x, r)
                })
                .filter(|(_, r)| !r.is_nan())
                .collect();
            ranks.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
            match tx.send((current_number, ranks)) {
                _ => return,
            };
        });
    }
}
