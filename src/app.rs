use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget};
use tui::Terminal;

use crate::event::{Event, Events};
use crate::query::QueryData;
use crate::search::rank_query;

#[derive(Default)]
pub struct App<'a> {
    data: Vec<QueryData>,
    search: Vec<String>,
    search_number: usize,
    ranked: (usize, Vec<(QueryData, f64)>),
    selected_item: Option<usize>,
    selected_search_box: usize,
    search_categories: Vec<Vec<&'a str>>,
    search_queued: bool,
}

impl<'a> From<Vec<QueryData>> for App<'a> {
    fn from(data: Vec<QueryData>) -> Self {
        let length = data.get(0).and_then(|x| Some(x.len())).unwrap_or(0);
        App {
            data: data,
            search: vec![String::new(); length],
            ..Default::default()
        }
    }
}

impl<'a> App<'a> {
    /// The main interactive app loop
    pub fn run(&mut self) -> std::io::Result<usize> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let stdout = MouseTerminal::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let events = Events::new();
        let (tx, rx) = mpsc::channel();

        terminal.hide_cursor()?;

        self.search_queued = true;

        loop {
            self.render(&mut terminal)?;

            match events.next().unwrap() {
                Event::Input(input) => match input {
                    Key::Ctrl('c') => {
                        break;
                    }
                    Key::Char('\n') | Key::Down => {
                        self.decrement_selected();
                    }
                    Key::Ctrl('k') | Key::Up => {
                        self.increment_selected();
                    }
                    Key::Ctrl('l') => {
                        break;
                    }
                    Key::Backspace => {
                        self.remove_letter();
                    }
                    Key::Ctrl('w') => {
                        self.remove_word();
                    }
                    Key::Char(x) => {
                        self.add_letter(x);
                    }
                    _ => {}
                },
                Event::Tick => {
                    if self.search_queued {
                        self.run_query(tx.clone());
                        self.search_queued = false;
                    }
                }
            }

            self.check_for_search_results(&rx);
        }
        match self.selected_item {
            Some(s) => match self.ranked.1.get(s) {
                Some((q, _)) => Ok(q.id),
                None => Err(std::io::Error::from(std::io::ErrorKind::Other)),
            },
            None => Err(std::io::Error::from(std::io::ErrorKind::Other)),
        }
    }

    fn increment_selected(&mut self) {
        self.selected_item = self.selected_item.map(|x| {
            if x + 1 >= self.ranked.1.len() {
                0
            } else {
                x + 1
            }
        });
    }

    fn decrement_selected(&mut self) {
        self.selected_item = self.selected_item.map(|x| {
            if x == 0 {
                self.ranked.1.len() - 1
            } else {
                x - 1
            }
        });
    }

    fn remove_letter(&mut self) {
        if let Some(ref mut s) = self.search.get_mut(self.selected_search_box) {
            s.pop();
            self.search_queued = true;
        }
    }

    fn add_letter(&mut self, letter: char) {
        if let Some(ref mut s) = self.search.get_mut(self.selected_search_box) {
            s.push(letter);
            self.search_queued = true;
        }
    }

    fn remove_word(&mut self) {
        let mut in_word = false;
        loop {
            if let Some(c) = self.search[0].pop() {
                if (c == '\n') | (c == '\t') | (c == ' ') {
                    if in_word {
                        self.search[0].push(c);
                        break;
                    }
                } else {
                    in_word = true;
                }
            } else {
                break;
            }
        }
        self.search_queued = true;
    }

    /// Run a search in a separate thread, sending the result back into an mpsc channel
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

    fn check_for_search_results(
        &mut self,
        channel: &mpsc::Receiver<(usize, Vec<(QueryData, f64)>)>,
    ) {
        for result in channel.try_iter() {
            if result.0 >= self.ranked.0 {
                self.ranked = result;
                let l = self.ranked.1.len();
                match self.selected_item {
                    Some(selected) => {
                        if selected >= l {
                            if l > 0 {
                                self.selected_item = Some(l - 1);
                            } else {
                                self.selected_item = None;
                            }
                        }
                    }
                    None => {
                        if l > 0 {
                            self.selected_item = Some(0);
                        }
                    }
                }
            }
        }
    }

    /// Render the interface
    fn render<Backend>(&self, terminal: &mut tui::Terminal<Backend>) -> std::io::Result<()>
    where
        Backend: tui::backend::Backend,
    {
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
                    &self
                        .ranked
                        .1
                        .iter()
                        .map(|(x, _)| x.clone().strings[0].clone())
                        .collect::<Vec<String>>(),
                )
                .select(self.selected_item)
                .highlight_symbol(">>")
                .render(&mut f, list_chunks[0]);

            Paragraph::new(vec![Text::raw(format!("{}", self.search[0]))].iter())
                .block(Block::default().borders(Borders::ALL).title("Search:"))
                .wrap(true)
                .render(&mut f, list_chunks[1]);
            if let Some(s) = self.selected_item {
                if let Some((i, _)) = self.ranked.1.get(s) {
                    Paragraph::new(i.into_paragraph().iter())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title("Selected item:"),
                        )
                        .wrap(true)
                        .render(&mut f, chunks[1]);
                }
            }
        })?;
        Ok(())
    }
}
