pub fn find_match(line: String, mut line_number: usize, target: &str, insensitive: bool, only: bool) -> Option<(usize, String)> {
    let target_lower = target.to_ascii_lowercase();
    let line_lower = &line.to_ascii_lowercase();
    if insensitive {
        if let Some(start) = line_lower.find(target_lower.as_str()) {
            if only {
                let end = start + target_lower.len();
                Some((line_number, line[start..end].to_string()))
            } else {
                Some((line_number, line))
            }
        } else {
            None
        }
    } else {
        if let Some(start) = line.find(target) {
            if only {
                let end = start + target.len();
                Some((line_number, line[start..end].to_string()))
            } else {
                Some((line_number, line))
            }
        } else {
            None
        }
    }
}

pub fn not_matched(line: String, mut line_number: usize, target: &str, insensitive: bool) -> Option<(usize, String)> {
    let target_lower = target.to_ascii_lowercase();
    let line_lower = &line.to_ascii_lowercase();
    if insensitive {
        match line_lower.find(target_lower.as_str()) {
            Some(_) => {
                None
            },
            None => {
                return Some((line_number, line));
            },
        }
    } else {
        match line.find(target) {
            Some(_) => {
                None
            },
            None => {
                return Some((line_number, line));
            },
        }
    }
}