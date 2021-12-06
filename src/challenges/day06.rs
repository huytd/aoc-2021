use std::collections::HashMap;
use crate::lib::read_lines_from;

fn read_input() -> Vec<u64> {
    let line = read_lines_from("./data/day06.txt").unwrap().next().unwrap().unwrap();
    line.split(",").map(|n| n.parse::<u64>().unwrap()).collect()
}

fn life(input: Vec<u64>, generations: usize) -> u64 {
    let mut life = [0u64; 9];
    for fish in input {
        life[fish as usize] += 1;
    }
    for _ in 0..generations {
        life.rotate_left(1);
        life[6] += life[8];
    }
    life.iter().sum()
}

pub fn part_one() {
    let input = read_input();
    println!("Day 06 - Part 01 = {}", life(input, 80));
}

pub fn part_two() {
    let input = read_input();
    println!("Day 06 - Part 02 = {}", life(input, 256));
}
