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
        Err(format!("{} isn't between 800 and 2000.", size))
    }
}

/// Check if format of date is ok
/// . (3) and - (0-1) for eventual BC date
pub fn validator_parse_date(date: String) -> Result<(), String> {
    let d: &str = &date.as_str();
    let items: Vec<_> = d.split(&['.', '-'][..]).collect();
    for item in &items {
        if !item.chars().all(char::is_numeric) {
            return Err(format!("{} is invalid because: {} isn't numeric. Please enter a date in format dd.mm.yyyy", date, item));
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
        Err(format!(
            "{} is invalid. Please enter a date in format dd.mm.yyyy.",
            date
        ))
    }
}

/// Check if format of time is ok
/// : (1-3) and after 3 ignore input
pub fn validator_parse_time(time: String) -> Result<(), String> {
    let d: &str = &time.as_str();
    let items: Vec<_> = d.split(&[':'][..]).collect();
    for (i, item) in items.clone().iter().enumerate() {
        if i > 2 {
            break;
        }
        if !item.chars().all(char::is_numeric) {
            return Err(format!("{} is invalid because: {} isn't numeric. Please enter time in format hh:mm or hh:mm:ss", time, item));
        }
    }
    if items.len() >= 1 {
        if items.len() == 1 && !time.chars().all(char::is_numeric) {
            Err(format!("{} is invalid. Please  enter a time in format hh:mm or hh:mm:ss", time        ))
        } else {
            Ok(())
        }
    } else {
        Err(format!(
            "{} is invalid. Please  enter a time in format hh:mm or hh:mm:ss",
            time
        ))
    }
}
