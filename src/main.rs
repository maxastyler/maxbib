extern crate rff;
extern crate serde;
extern crate serde_yaml;
extern crate termion;
extern crate tui;

use serde_yaml::Value;
use std::io::Result;

pub mod query;
pub mod search;

use crate::query::*;
use crate::search::{rank_query, rank_query_weighted};

fn main() -> serde_yaml::Result<()> {
    let v = serde_yaml::from_str::<Value>(include_str!("/home/max/git/maxbib/references.yaml"))?;
    let s = &v[0];
    let arguments = vec![vec!["issue", "volume", "full_journal_title", "author"], vec!["title"], vec!["abstract"]];
    let searches = vec!["ho"];
    let weights: Vec<f64> = vec![0.11, 100.0, 20.0, 0.001];
    println!("{:?}", QueryData::build(s, &arguments));
    for _ in 0..100000 {
        rank_query(&QueryData::build(s, &arguments), &searches);
    }
    println!("{:?}", rank_query_weighted(&QueryData::build(s, &arguments), &searches, &weights));
    Ok(())
}
