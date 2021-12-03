use crate::lib::read_lines_from;

fn read_input() -> Vec<String> {
    let mut result = vec![];
    if let Ok(lines) = read_lines_from("./data/day03.txt") {
        for line in lines {
            result.push(line.unwrap());
        }
    }
    result
}

pub fn part_one() {
    let lines = read_input();
    let cols = lines[0].len();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for col in 0..cols {
        let mut count = 0;
        for line in &lines {
            let c = line.chars().nth(col).unwrap();
            if c.eq(&'1') {
                count += 1;
            }
        }
        let (most, least) = if count > lines.len() / 2 { ('1', '0') } else { ('0', '1') };
        gamma.push(most);
        epsilon.push(least);
    }
    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
    println!("Day 03 - Part 01 = {}", gamma * epsilon);
}

pub fn part_two() {
    let lines = read_input();
    let cols = lines[0].len();
    let mut oxygen = lines.clone();
    let mut carbonic = lines.clone();
    for col in 0..cols {
        let mut ones = 0;
        let mut zeroes = 0;
        for line in &oxygen {
            let c = line.chars().nth(col).unwrap();
            if c.eq(&'1') {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }
        let criteria = if ones >= zeroes { '1' } else { '0' };
        oxygen.retain(|line| line.chars().nth(col).unwrap().eq(&criteria));
        if oxygen.len() == 1 {
            break;
        }
    }
    for col in 0..cols {
        let mut ones = 0;
        let mut zeroes = 0;
        for line in &carbonic {
            let c = line.chars().nth(col).unwrap();
            if c.eq(&'1') {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }
        let criteria = if ones >= zeroes { '0' } else { '1' };
        carbonic.retain(|line| line.chars().nth(col).unwrap().eq(&criteria));
        if carbonic.len() == 1 {
            break;
        }
    }
    let oxygen = i32::from_str_radix(&oxygen.first().unwrap(), 2).unwrap();
    let carbonic = i32::from_str_radix(&carbonic.first().unwrap(), 2).unwrap();
    println!("Day 03 - Part 02 = {}", oxygen * carbonic);
}
