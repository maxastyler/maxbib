//! The query module, which contains functions to do with Queries, which are searched on
use serde_yaml::Value;

use tui::widgets::Text;

/// QueryData is the set of strings that are to be searched through
/// The given strings are built from a `Vec<Vec<&str>>` where the inner vector
/// contains the terms which are to be grouped together into the different strings
/// and the outer vector contains the number of different strings that will be built
#[derive(Debug, PartialEq, Clone)]
pub struct QueryData {
    /// The built query
    pub strings: Vec<String>,
    /// The id of the bib entry that this query corresponds to
    pub id: usize,
}

impl QueryData {
    /// Build a `QueryData` entry from a `serde_yaml` `Mapping` (containing the reference data) and a `Vec` containing `Vec<&str>` which denote the arguments to be combined for each element
    pub fn build(id: usize, entry: &Value, arguments: &Vec<Vec<&str>>) -> QueryData {
        let strings: Vec<String> = arguments
            .iter()
            .map(|strings| {
                let mut s = strings
                    .iter()
                    .map(|s| match entry.get(s) {
                        Some(v) => v.as_str().unwrap_or(""),
                        None => "",
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                s.truncate(1024); // truncate search string to 1024 characters, as rff has that as a maximum
                s
            })
            .collect();
        QueryData {
            strings: strings,
            id: id,
        }
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn into_paragraph(&self) -> Vec<Text> {
        self.strings
            .iter()
            .map(|s| Text::raw(format!("{}\n", s)))
            .collect::<Vec<_>>()
    }
}

impl Default for QueryData {
    fn default() -> Self {
        QueryData {
            strings: vec![],
            id: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::QueryData;
    use serde_yaml::{Mapping, Value};
    use std::iter::FromIterator;

    #[test]
    fn querydata_builds_correctly_multiples() {
        let v = Value::Mapping(Mapping::from_iter(vec![
            (
                Value::String(String::from("title")),
                Value::String(String::from("a test title")),
            ),
            (
                Value::String(String::from("author")),
                Value::String(String::from("arthur grunp")),
            ),
        ]));
        let query = QueryData::build(
            0,
            &v,
            &vec![vec!["title", "author"], vec!["author", "title", "title"]],
        );
        assert_eq!(
            query,
            QueryData {
                strings: vec![
                    String::from("a test title\narthur grunp"),
                    String::from("arthur grunp\na test title\na test title")
                ],
                id: 0
            }
        )
    }

    #[test]
    fn querydata_build_truncates_to_1024_chars() {
        let long_str = Value::String((0..1050).map(|_| 'a').collect::<String>());
        let shorter_str = Value::String((0..1024).map(|_| 'a').collect::<String>());
        let v = Value::Mapping(Mapping::from_iter(vec![(
            Value::String(String::from("long")),
            long_str,
        )]));
        let query = QueryData::build(0, &v, &vec![vec!["long", "long"]]);
        assert_eq!(query.strings, vec![shorter_str]);
    }

}
