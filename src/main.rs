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
    let v_iter = v.as_sequence().unwrap().iter().cycle().take(100_000);
    // for (i, _) in v_iter.enumerate() {
    //     // println!("{:?}", i);
    // }
    // println!("iterator length: {}", v_iter.len());
    // let arguments = vec![vec!["title", "journal", "year"]; 3];
    let arguments = vec![vec!["title", "author"], vec!["journal"], vec!["year"]];
    // let searches = vec!["entanglement"; 3];
    let searches = vec!["", "", ""];
    // let weights: Vec<f64> = vec![1.0, 1.0];
    let queries: Vec<_> = v_iter.map(|x| QueryData::build(x, &arguments)).collect();
    let mut ranks: Vec<_> = queries
        .iter()
        .map(|x| (x, rank_query(x, &searches)))
        .filter(|(_, r)| !r.is_nan())
        .collect();
    ranks.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    // println!("{:?}", ranks[0]);
    for (i, _) in &ranks[0..1] {
        for s in &i.strings {
            println!("{}", s);
        }
    }
    Ok(())
}
