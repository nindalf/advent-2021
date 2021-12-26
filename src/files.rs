use anyhow::Error;
use std::io::BufRead;

#[allow(dead_code)]
pub fn read_numbers(file_name: &str) -> Result<Vec<i32>, Error> {
    let file = std::fs::File::open(file_name)?;
    Ok(std::io::BufReader::new(file)
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect())
}

#[allow(dead_code)]
pub fn read_lines(file_name: &str) -> Result<Vec<String>, Error> {
    let file = std::fs::File::open(file_name)?;
    Ok(std::io::BufReader::new(file)
        .lines()
        .filter_map(|line_result| line_result.ok())
        .collect())
}

#[allow(dead_code)]
pub fn read_string(file_name: &str) -> Result<String, Error> {
    Ok(std::fs::read_to_string(file_name)?)
}
