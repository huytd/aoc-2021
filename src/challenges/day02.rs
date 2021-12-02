use crate::lib::read_lines_from;

enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32)
}

fn read_input() -> Vec<Direction> {
    let mut result = vec![];
    if let Ok(lines) = read_lines_from("./data/day02.txt") {
        for line in lines {
            if let Ok(line) = line {
                let mut split = line.split(" ");
                let cmd = split.next().unwrap();
                let num = split.next().unwrap().parse::<i32>().unwrap();
                let parsed = match cmd {
                    "forward" => Direction::Forward(num),
                    "up" => Direction::Up(num),
                    _ => Direction::Down(num),
                };
                result.push(parsed);
            }
        }
    }
    result
}

pub fn part_one() {
    let commands = read_input();
    let mut x = 0;
    let mut y = 0;
    for cmd in commands {
        match cmd {
            Direction::Forward(mx) => x += mx,
            Direction::Up(my) => y -= my,
            Direction::Down(my) => y += my,
        }
    }
    let result = x * y;
    println!("Day 02 - Part 01 = {}", result);
}

pub fn part_two() {
    let commands = read_input();
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for cmd in commands {
        match cmd {
            Direction::Forward(mx) => {
                x += mx;
                y += aim * mx;
            },
            Direction::Up(my) => {
                aim -= my;
            },
            Direction::Down(my) => {
                aim += my;
            }
        }
    }
    let result = x * y;
    println!("Day 02 - Part 02 = {}", result);
}
