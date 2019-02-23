use glob::{glob, PatternError};

pub fn load_library(path: &str) -> Result<Vec<serde_yaml::Value>, PatternError> {
    Ok(glob(&format!("{}/**/*.yaml", path))?
        .filter_map(|p| match p {
            Ok(path) => {
                let f = std::fs::File::open(path).unwrap();
                serde_yaml::from_reader::<_, serde_yaml::Value>(f).ok()
            }
            Err(_) => None,
        })
        .collect::<Vec<_>>())
}
