extern crate rff;
extern crate serde;
extern crate serde_yaml;
extern crate termion;
extern crate tui;

use serde_yaml::Value;
use std::io::Result;

pub mod event;
pub mod query;
pub mod search;
pub mod app;

use crate::query::*;
use crate::app::App;

fn main() -> Result<()> {
    let v = serde_yaml::from_str::<Value>(include_str!("/home/max/git/maxbib/references.yaml")).unwrap();
    let v_iter = v.as_sequence().unwrap().iter().cycle().take(100_00);
    let arguments = vec![vec!["title", "author"], vec!["journal"], vec!["year"]];
    let queries: Vec<_> = v_iter.map(|x| QueryData::build(x, &arguments)).collect();
    let mut a = App::from(queries);
    a.run()
}
