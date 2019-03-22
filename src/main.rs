extern crate glob;
extern crate rff;
extern crate serde;
extern crate serde_yaml;
extern crate termion;
extern crate tui;

use std::io::Result;

pub mod app;
pub mod event;
pub mod library;
pub mod query;
pub mod search;

use crate::app::App;
use crate::library::load_library;
use crate::query::*;

fn main() -> Result<()> {
    let lib = load_library("/home/max/papers/").unwrap();
    let arguments = vec![vec!["title"], vec!["author"], vec!["journal"], vec!["year"], vec!["abstract"]];
    // let arguments = vec![vec!["title"], vec!["year"]];
    let queries: Vec<_> = lib.iter()
        .map(|(i, x)| QueryData::build(*i, x, &arguments))
        .collect();
    let mut a = App::from((queries, arguments));
    let selected = a.run()?;
    println!("{:?}", lib.get(selected).unwrap().1.get(&"files").unwrap());
    Ok(())
}
