use std::{fs::File, io::{self, BufRead, BufReader}};

pub fn read_lines_from(input: &str) -> std::io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(input)?;
    Ok(io::BufReader::new(file).lines())
}
