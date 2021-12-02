use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
use std::str::FromStr;
use anyhow::{self, Context, Error};

pub const DEFAULT_INPUT_FOLDER: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "inputs");

#[cfg(test)]
pub fn example_input(day: u32) -> String {
    format!("{}/{}_example.txt", DEFAULT_INPUT_FOLDER, day)
}

pub fn parse_file_line_by_line<T: FromStr>(path: &str) -> anyhow::Result<Vec<T>>
{
    let file = File::open(path).context("Couldn't open file")?;
    let reader = BufReader::new(file);

    reader.lines()
        .map(line_to_parsed)
        .collect()
}

fn line_to_parsed<U: FromStr>(line: std::io::Result<String>) -> anyhow::Result<U>
{
    let l = line?;
    l.parse::<U>().map_err(|_| Error::msg(format!("Couldn't parse: {}", l)))
}
