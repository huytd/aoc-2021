use std::fs::File;
use std::io::{self, BufRead};

fn open_input() -> std::io::Result<Vec<i32>> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file).lines();
    let mut result = vec![];
    for line in reader {
        let num = line.unwrap().parse::<i32>().unwrap();
        result.push(num);
    }
    Ok(result)
}

#[allow(dead_code)]
fn first() {
    let input = open_input().unwrap();
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i-1] {
            count += 1;
        }
    }
    println!("{}", count);
}

fn second() {
    let input = open_input().unwrap();
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
    println!("{}", count);
}

fn main() {
    // first();
    second();
}

