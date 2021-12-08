use std::i32::MAX;
use crate::lib::read_lines_from;

struct Input { }

fn read_input() -> Vec<i32> {
    let line = read_lines_from("./data/day07.txt").unwrap().next().unwrap().unwrap();
    line.split(",").map(|n| n.parse::<i32>().unwrap()).collect()
}

pub fn part_one() {
    let input = read_input();
    let mut min_fuel = MAX;
    for target in 0..input.len() as i32 {
        let fuel: i32 = input.iter().map(|val| (val - target).abs()).sum();
        min_fuel = min_fuel.min(fuel);
    }
    println!("Day 07 - Part 01 = {}", min_fuel);
}

pub fn part_two() {
    let input = read_input();
    let mut min_fuel = MAX;
    for target in 0..input.len() as i32 {
        let fuel: i32 = input.iter().map(|val| {
            let mut cost = (val - target).abs();
            for i in 1..(cost + 0) {
                cost += i;
            }
            return cost;
        }).sum();
        min_fuel = min_fuel.min(fuel);
    }
    println!("Day 07 - Part 02 = {}", min_fuel);
}
