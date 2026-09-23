use std::error::Error;
use std::fs::{self, *};
use std::io::ErrorKind;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::usize;

use crate::matching::{strict::{find_match_strict, not_matched_strict}, relaxed::{find_match, not_matched}};
use crate::errors::{FileReadError};

pub fn find_lines_individual(
        file: &PathBuf,
        target: &str,
        whole: bool,
        insensitive: bool, 
        only: bool, 
        invert: bool, 
        max_count: Option<usize>,
        context: Option<usize>,
    ) -> Result<Vec<(usize, String)>, FileReadError> {

    let mut matches = Vec::new();

    let context_bool = context.is_some();

    let mut all_lines: Vec<(usize, String)> = Vec::new();

    for (line_number, line_result) in read_lines(file)? {
        let line = line_result?;

        //check this
        all_lines.push((line_number, line.clone()));

        if max_count.is_some() && matches.len() == max_count.unwrap() {
            break;
        }

        if whole {
            if invert {
                if let Some(found) = not_matched_strict(line, line_number, target, insensitive) {
                    matches.push(found);
                }
            } else {
                if let Some(found) = find_match_strict(line, line_number, target, insensitive, only) {
                    matches.push(found);
                }
            }
        } else {
            if invert {
                if let Some(found) = not_matched(line, line_number, target, insensitive) {
                    matches.push(found);
                }
            } else {
                if let Some(found) = find_match(line, line_number, target, insensitive, only) {
                    matches.push(found);
                }
            }
        }
    }

    if context_bool {
        let lower = context.unwrap();
        let upper = lower;
        let ranges = bingus(&matches, &all_lines, lower, upper)?;
        dbg!(&ranges);
        matches.clear();

        for range in ranges {
            let get = all_lines.get(range);
            if let Some(slice_with_context) = get {
                matches.extend(slice_with_context.iter().cloned());
            };
        }

        return Ok(matches);
    } else {
        return Ok(matches);
    }

}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = (usize, io::Result<String>)>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().enumerate().map(|(mut line_number, line)| {(line_number + 1, line)}))
}

fn retrieve(matches: &Vec<(usize, String)>, all_lines: &Vec<(usize, String)>, lower: usize, upper: usize) -> Result<Vec<std::ops::Range<usize>>, FileReadError> {
    //this is with actual index!!!! -- delete index line if want line number instead
    if matches.is_empty() {
        return Ok(Vec::new());
    }

    const CLAMP_MIN: usize = usize::MIN;
    let upper_plus_one = upper.saturating_add(1);
    let Some(clamp_max) = all_lines.len().checked_sub(1).filter(|c| *c >= CLAMP_MIN + 1) else {
        return Err(FileReadError::ShortSlice);
    };
    let mut result = Vec::new();
    for (index, _) in matches {
        //use real_index if want actual index -- test first
        let real_index = index.saturating_sub(1);
        let start = real_index.saturating_sub(lower).clamp(CLAMP_MIN, clamp_max.saturating_add(1));
        let end = real_index.saturating_add(upper_plus_one).clamp(CLAMP_MIN, clamp_max.saturating_add(1));
        result.push(start..end)
    }

    Ok(result)
}

fn bingus(matches: &Vec<(usize, String)>, all_lines: &Vec<(usize, String)>, lower: usize, upper: usize) -> Result<Vec<std::ops::Range<usize>>, FileReadError> {
    let ranges = retrieve(matches, all_lines, lower, upper)?;
    dbg!(&ranges);

    let mut result = Vec::new();
    let mut iter = ranges.into_iter();
    if let Some(mut r) = iter.next() {
        for r2 in iter {
            let overlap = r2.start <= r.end;
            if overlap {
                r.end = r.end.max(r2.end);
            } else {
                result.push(r);
                r = r2;
            }
        }
        result.push(r);
    }

    Ok(result)
}

//find a way to get index in all_lines, then should be good with safe_retrieve, will just have to like build a new vec from all the returns
//from the fn and collect

//Look into merging , lower..upper+1 in pairs? figure out algo