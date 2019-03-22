//! Functions related to loading things from library folders
use glob::{glob, PatternError};

/// Load a set of yaml values from all the yaml files in a given path
pub fn load_library(path: &str) -> Result<Vec<(usize, serde_yaml::Value)>, PatternError> {
    Ok(glob(&format!("{}/**/*.yaml", path))?
        .filter_map(|p| match p {
            Ok(path) => {
                let f = std::fs::File::open(path).unwrap();
                serde_yaml::from_reader::<_, serde_yaml::Value>(f).ok()
            }
            Err(_) => None,
        })
        .enumerate().collect::<Vec<_>>())
}
