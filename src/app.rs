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

        terminal.hide_cursor()?;
        let mut selected = 0;
        let mut ranked: (usize, Vec<(QueryData, f64)>) = (0, vec![]);

        self.run_query(tx.clone());

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
                        &ranked
                            .1
                            .iter()
                            .map(|(x, _)| x.clone().strings[0].clone())
                            .collect::<Vec<String>>(),
                    )
                    .select(Some(selected))
                    .highlight_symbol(">>")
                    .render(&mut f, list_chunks[0]);

                Paragraph::new(vec![Text::raw(format!("{}", self.search[0]))].iter())
                    .block(Block::default().borders(Borders::ALL).title("Search:"))
                    .render(&mut f, list_chunks[1]);
                if let Some((i, _)) = ranked.1.get(selected) {
                    Paragraph::new(i.into_paragraph().iter())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title("Selected item:"),
                        )
                        .render(&mut f, chunks[1]);
                }
            })?;

            match events.next().unwrap() {
                Event::Input(input) => match input {
                    Key::Ctrl('c') => {
                        break;
                    }
                    Key::Char('\n') | Key::Down => {
                        selected += 1;
                        if selected >= ranked.1.len() {
                            selected = 0;
                        }
                    }
                    Key::Ctrl('k') | Key::Up => {
                        if selected == 0 {
                            selected = ranked.1.len() - 1;
                        } else {
                            selected -= 1
                        }
                    }
                    Key::Ctrl('l') => {
                        break;
                    }
                    Key::Backspace => {
                        self.search[0].pop();
                        self.run_query(tx.clone())
                    }
                    Key::Char(x) => {
                        self.search[0].push(x);
                        self.run_query(tx.clone())
                    }
                    _ => {}
                },
                _ => (),
            }

            for result in rx.try_iter() {
                if result.0 >= ranked.0 {
                    ranked = result;
                }
            }
        }
        Ok(())
    }

    pub fn run_query(&mut self, tx: mpsc::Sender<(usize, Vec<(QueryData, f64)>)>) {
        let data = self.data.clone();
        let search_strings = self.search.clone();
        let current_number = self.search_number;
        thread::spawn(move || {
            let mut ranks: Vec<_> = data
                .into_iter()
                .map(|x| {
                    let r = rank_query(&x, &search_strings.iter().map(|x| x.as_str()).collect());
                    (x, r)
                })
                .filter(|(_, r)| !r.is_nan())
                .collect();
            ranks.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
            match tx.send((current_number, ranks)) {
                _ => return,
            };
        });
        self.search_number += 1;
    }
}
