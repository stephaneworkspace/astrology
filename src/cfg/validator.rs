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

/// Check if size is ok
pub fn parse_size(size: String) -> Result<(), String> {
    let i: u32 = size.parse::<u32>().unwrap();
    if i >= 800 && i <= 2000 {
        Ok(())
    } else {
        Err(format!("The number: {} isn't between 800 and 2000", size))
    }
}
