use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use chrono::NaiveDateTime;

pub fn parse_file(file: &str) -> Result<HashMap<u64, Vec<Entry>>, std::io::Error> {
    let file = File::open(file)?;
    let file = BufReader::new(file);

    // key: max count, value: value series
    let mut parsed: HashMap<u64, Vec<Entry>> = HashMap::new();
    let mut last_timestamp: Option<i64> = None;
    for (line_idx, line) in file.lines().enumerate() {
        let line = line?;
        if line.starts_with("[") {
            last_timestamp = Some(parse_time(&line));
        } else if line.starts_with("Iter") {
            if last_timestamp.is_none() {
                eprintln!("Bad file: Skipping line because there is no timestamp in context");
                continue;
            }
            let values = parse_entry(&line);
            let entry = Entry {
                time: last_timestamp.expect("Check above"),
                value: values.0
            };
            parsed.entry(values.1)
                .and_modify(|vec| vec.push(entry.clone()))
                .or_insert(vec![entry]);
        } else {
            log::info!(target: "parser", "unrecognized line {}", line_idx)
        }
    }

    Ok(parsed)
}

// Parses time of format `[Sun Apr 14 17:05:29 2024]` to unix seconds.
fn parse_time(line: &String) -> i64 {
    let line = line.trim_matches(|c| c == '[' || c == ']');
    let time = NaiveDateTime::parse_from_str(line, "%a %b %d %H:%M:%S %Y").expect("file format incorrect");
    time.and_utc().timestamp()
}

fn parse_entry(line: &String) -> (u64, u64) {
    let values: Vec<&str> = line[10..].split(" / ").collect();
    let val = u64::from_str(values[0]).expect("Unparsable number");
    let div = u64::from_str(values[1]).expect("Unparsable number");
    (val, div)
}

#[derive(Clone)]
pub struct Entry {
    /// Unix timestamp seconds.
    pub time: i64,
    pub value: u64,
}