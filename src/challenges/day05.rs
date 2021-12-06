use std::collections::{HashMap, HashSet};

use crate::lib::read_lines_from;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point
}

impl Line {
    pub fn get_points(&self) -> Vec<Point> {
        let mut result = vec![];
        if self.start.x == self.end.x {
            // horizontal line
            let mut pos = [self.start.y, self.end.y]; pos.sort();
            for y in pos[0]..=pos[1] {
                result.push(Point{ x: self.start.x, y });
            }
        } else if self.start.y == self.end.y {
            // vertical line
            let mut pos = [self.start.x, self.end.x]; pos.sort();
            for x in pos[0]..=pos[1] {
                result.push(Point{ y: self.start.y, x });
            }
        } else {
            // diagonal lines
            // Just use the navie line drawing algorithm from wikipedia:
            // https://en.m.wikipedia.org/wiki/Line_drawing_algorithm
            let dx = self.end.x - self.start.x;
            let dy = self.end.y - self.start.y;
            let mut pos = [self.start.x, self.end.x]; pos.sort();
            for x in pos[0]..=pos[1] {
                let y = self.start.y + dy * (x - self.start.x) / dx;
                result.push(Point { x, y });
            }
        }
        result
    }
}

#[derive(Debug)]
struct Input {
    lines: Vec<Line>
}

fn to_num(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn read_input() -> Input {
    let lines = read_lines_from("./data/day05.txt").unwrap();
    let mut result = vec![];
    for line in lines {
        if let Ok(line) = line {
            let (left, right) = line.split_once(" -> ").unwrap();
            let (x1, y1) = left.split_once(",").unwrap();
            let (x2, y2) = right.split_once(",").unwrap();
            result.push(Line {
                start: Point { x: to_num(x1), y: to_num(y1) },
                end: Point { x: to_num(x2), y: to_num(y2) }
            });
        }
    }
    Input {
        lines: result
    }
}

pub fn part_one() {
    let input = read_input();
    let lines: Vec<Line> = input.lines.into_iter().filter(|line| line.start.x == line.end.x || line.start.y == line.end.y).collect();
    let mut points: Vec<Point> = vec![];
    let mut hash = HashMap::<Point, i32>::new();
    for line in lines {
        points.append(&mut line.get_points());
    }
    for point in points {
        if let Some(count) = hash.get_mut(&point) {
            *count += 1;
        } else {
            hash.insert(point, 1);
        }
    }
    let found = hash.iter().filter(|(_, &value)| value > 1).count();
    println!("Day 05 - Part 01 = {}", found);
}

pub fn part_two() {
    let input = read_input();
    let lines: Vec<Line> = input.lines;
    let mut points: Vec<Point> = vec![];
    let mut hash = HashMap::<Point, i32>::new();
    for line in lines {
        points.append(&mut line.get_points());
    }
    for point in points {
        if let Some(count) = hash.get_mut(&point) {
            *count += 1;
        } else {
            hash.insert(point, 1);
        }
    }
    let found = hash.iter().filter(|(_, &value)| value > 1).count();
    println!("Day 05 - Part 02 = {}", found);
}
