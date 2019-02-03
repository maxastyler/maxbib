extern crate rff;
extern crate serde;
extern crate serde_yaml;
extern crate termion;
extern crate tui;

use serde_yaml::Value;
use std::io::Result;

mod query;

use crate::query::*;

fn main() -> serde_yaml::Result<()> {
    let v = serde_yaml::from_str::<Value>(include_str!("/home/max/git/maxbib/references.yaml"))?;
    let s = &v[0];
    let arguments = vec![vec!["author", "pauthor"]];
    println!("{:?}", QueryData::build(s, &arguments));
    Ok(())
}
