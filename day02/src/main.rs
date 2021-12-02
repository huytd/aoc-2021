use std::{fs::File, io::{self, BufRead}};

#[derive(Debug)]
enum Direction {
    Forward(isize),
    Up(isize),
    Down(isize)
}

fn open_input() -> std::io::Result<Vec<Direction>> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file).lines();
    let mut result = vec![];
    for line in reader {
        if let Ok(line) = line {
            let mut split = line.split(" ");
            let cmd = split.next().unwrap();
            let num = split.next().unwrap().parse::<isize>().unwrap();
            let parsed = match cmd {
                "forward" => Direction::Forward(num),
                "up" => Direction::Up(num),
                _ => Direction::Down(num),
            };
            result.push(parsed);
        }
    }
    Ok(result)
}

#[allow(dead_code)]
fn part_one() {
    if let Ok(commands) = open_input() {
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
        println!("{}", result);
    }
}

fn part_two() {
    if let Ok(commands) = open_input() {
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
        println!("{}", result);
    }
}

fn main() {
    // part_one();
    part_two();
}
