extern crate rand;
extern crate rff;
extern crate serde;
extern crate serde_yaml;
extern crate termion;
extern crate tui;
#[macro_use]
extern crate serde_derive;

use rand::prelude::*;
use rand::thread_rng;
use rff::match_and_score;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

const SEARCH_STRING: &str =  "app";

#[derive(Serialize, Deserialize, Debug)]
enum Component {
    Str,
    List { dic: HashMap<String, Component> },
    Items { items: Vec<Component> },
}

#[derive(Debug)]
struct Reference {
    data: HashMap<String, String>,
}

impl Reference {
    fn new(rng: &mut ThreadRng) -> Reference {
        let mut data = HashMap::new();
        data.insert(
            String::from("title"),
            (0..100)
                .map(|_| rng.gen_range::<u8, _, _>(97, 122) as char)
                .collect::<String>(),
        );
        data.insert(
            String::from("abstract"),
            (0..900)
                .map(|_| rng.gen_range::<u8, _, _>(97, 122) as char)
                .collect::<String>(),
        );
        // data.insert(String::from("abstract"), String::from_utf8(abs).unwrap());
        Reference { data: data }
    }

    fn create_str(&self) -> String {
        format!(
            "{}\n{}",
            self.data.get("title").unwrap(),
            self.data.get("abstract").unwrap()
        )
    }
}

fn main() -> Result<(), io::Error> {
    let info = include_str!("/home/max/papers/0306e35006dbe443a6673c4d872fe753-shannon-c.-e./info.yaml");
    let mut rng = thread_rng();
    let references: Vec<_> = (0..1000).map(|_| Reference::new(&mut rng)).collect();
    let static_strings: Vec<String> = references
        .iter()
        .map(|i| i.create_str())
        .collect();
    let mut searches: Vec<(&String, f64)> = static_strings
        .iter()
        .map(|s| {
            let f = match_and_score(SEARCH_STRING, s.as_str()).unwrap_or((SEARCH_STRING, std::f64::NAN)).1;
            (s, f)
        })
        .filter(|x| !x.1.is_nan())
        .collect();
    println!("{:?}", searches.len());
    searches.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    for i in searches {
        println!("{}", i.1);
    }
    let v: serde_yaml::Value = serde_yaml::from_str(info).unwrap();
    for i in v.as_mapping().unwrap().iter() {
        println!("{:?}", i);
    }
    Ok(())
}
