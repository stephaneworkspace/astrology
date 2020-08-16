use std::path::Path;

/// Check if the path for the swissephem files exist
pub fn validator_parse_path(path: String) -> Result<(), String> {
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
pub fn validator_parse_size(size: String) -> Result<(), String> {
    let i: u32 = size.parse::<u32>().unwrap();
    if i >= 800 && i <= 2000 {
        Ok(())
    } else {
        Err(format!("The number: {} isn't between 800 and 2000.", size))
    }
}

/// Check if format of date is ok
/// . (3) and - (0-1) for eventual BC date
pub fn validator_parse_date(date: String) -> Result<(), String> {
    let d: &str = &date.as_str();
    let items: Vec<_> = d.split(&['.', '-'][..]).collect();
    for item in &items {
        if !item.chars().all(char::is_numeric) {
            return Err(format!("The date: {} is invalid because: {} isn't numeric. Please enter a date in format dd.mm.yyyy", date, item));
        }
    }
    let mut bc_4_item = 0;
    for i in 0..date.clone().len() {
        let str_bytes = date.clone().as_bytes()[i] as char;
        if str_bytes == '-' {
            bc_4_item += 1;
        }
    }
    if (items.len() == 3 && bc_4_item == 0)
        || (items.len() == 4 && bc_4_item == 1)
    {
        Ok(())
    } else {
        Err(format!("The date: {} is invalid. Please enter a date in format dd.mm.yyyy.", date))
    }
}
