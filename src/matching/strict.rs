use regex::Regex;

pub fn find_match_strict(line: String, mut line_number: usize, target: &str, insensitive: bool, only: bool) -> Option<(usize, String)> {
    let strict = format!(r"\b{}\b", target);
    if insensitive {
        let strict_lower = strict.to_ascii_lowercase();
        let line_lower = line.to_ascii_lowercase();
        let re = Regex::new(strict_lower.as_str()).unwrap();
        match re.find(&line_lower) {
            Some(matched) => {
                if only {
                    let start = matched.start();
                    let end = matched.end();
                    let current_target = line[start..end].to_string();
                    return Some((line_number, current_target));
                } else {
                    return Some((line_number, line));
                }
            },
            None => {
                return None;
            },
        }
    } else {
        let re = Regex::new(strict.as_str()).unwrap();
        match re.find(&line) {
            Some(matched) => {
                if only {
                    let start = matched.start();
                    let end = matched.end();
                    let current_target = line[start..end].to_string(); 
                    return Some((line_number, current_target));
                } else {
                    return Some((line_number, line));
                }
            },
            None => {
                return None;
            },
        }
    }
}

pub fn not_matched_strict(line: String, mut line_number: usize, target: &str, insensitive: bool) -> Option<(usize, String)> {
    let strict = format!(r"\b{}\b", target);
    if insensitive {
        let strict_lower = strict.to_ascii_lowercase();
        let line_lower = line.to_ascii_lowercase();
        let re = Regex::new(strict_lower.as_str()).unwrap();
        match re.find(&line_lower) {
            Some(_) => {
                None
            },
            None => {
                return Some((line_number, line));
            },
        }
    } else {
        let re = Regex::new(strict.as_str()).unwrap();
        match re.find(&line) {
            Some(_) => {
                None
            },
            None => {
                return Some((line_number, line));
            },
        }
    }
}
