use std::path::Path;

/// Check if the path for the swissephem files exist
pub fn parse_path(path: String) -> Result<(), String> {
    if Path::new(path.clone().as_str()).exists() {
        Ok(())
    } else {
        Err(format!(
            "The path for your swiss emphem files don't exist: {}",
            path
        ))
    }
}
