use crate::lib::read_lines_from;

fn read_input() -> Vec<i32> {
    let mut result = vec![];
    if let Ok(lines) = read_lines_from("./data/day01.txt") {
        for line in lines {
            let num = line.unwrap().parse::<i32>().unwrap();
            result.push(num);
        }
    }
    result
}

pub fn part_one() {
    let input = read_input();
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i-1] {
            count += 1;
        }
    }
    println!("Day 01 - Part 01 = {}", count);
}

pub fn part_two() {
    let input = read_input();
    let mut count = 0;
    let mut prev = -1;
    for i in 0..input.len() - 2 {
        let sum = input[i] + input[i+1] + input[i+2];
        if prev == -1 {
            prev = sum;
        } else {
            if sum > prev {
                count += 1;
            }
            prev = sum;
        }
    }
    println!("Day 01 - Part 02 = {}", count);
}
