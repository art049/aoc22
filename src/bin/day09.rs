#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "09";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

#[derive(Debug, Clone)]
enum Move {
    Up,
    Down,
    Right,
    Left,
    Diagonal(Box<Move>, Box<Move>),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos(i32, i32);

fn parse_moves(lines: &Vec<String>) -> Vec<Move> {
    lines
        .iter()
        .map(|l| -> Vec<Move> {
            let parts = l.split(" ").collect_vec();
            let n = parts[1].parse::<usize>().expect("invalid move length");
            let m = match parts[0] {
                "U" => Move::Up,
                "L" => Move::Left,
                "R" => Move::Right,
                "D" => Move::Down,
                _ => panic!("Unknown move"),
            };
            vec![m; n]
        })
        .flatten()
        .collect_vec()
}

fn distance(Pos(x1, y1): Pos, Pos(x2, y2): Pos) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn apply_move(pos: Pos, m: &Move) -> Pos {
    match m {
        Move::Left => Pos(pos.0 - 1, pos.1),
        Move::Up => Pos(pos.0, pos.1 + 1),
        Move::Right => Pos(pos.0 + 1, pos.1),
        Move::Down => Pos(pos.0, pos.1 - 1),
        Move::Diagonal(m1, m2) => apply_move(apply_move(pos, m1), m2),
    }
}

fn get_knot_move(head_pos: Pos, knot_pos: Pos) -> Option<Move> {
    let d = distance(head_pos, knot_pos);
    if d <= 1 {
        None
    } else if head_pos.1 == knot_pos.1 {
        if head_pos.0 > knot_pos.0 {
            Some(Move::Right)
        } else {
            Some(Move::Left)
        }
    } else if head_pos.0 == knot_pos.0 {
        if head_pos.1 > knot_pos.1 {
            Some(Move::Up)
        } else {
            Some(Move::Down)
        }
    } else if d > 2 {
        Some(Move::Diagonal(
            if head_pos.0 > knot_pos.0 {
                Box::new(Move::Right)
            } else {
                Box::new(Move::Left)
            },
            if head_pos.1 > knot_pos.1 {
                Box::new(Move::Up)
            } else {
                Box::new(Move::Down)
            },
        ))
    } else {
        None
    }
}

fn part1(lines: &Vec<String>) -> u32 {
    let moves = parse_moves(lines);
    let mut tail_visited = HashSet::new();
    let mut head_pos = Pos(0, 0);
    let mut tail_pos = Pos(0, 0);
    tail_visited.insert(tail_pos);
    for m in moves {
        head_pos = apply_move(head_pos, &m);
        if let Some(tail_move) = get_knot_move(head_pos, tail_pos) {
            tail_pos = apply_move(tail_pos, &tail_move);
            tail_visited.insert(tail_pos);
        }
    }
    tail_visited.len() as u32
}

const KNOTS: usize = 10;

fn part2(lines: &Vec<String>) -> u32 {
    let moves = parse_moves(lines);
    let mut tail_visited = HashSet::new();
    let mut knots = [Pos(0, 0); KNOTS];
    tail_visited.insert(knots[KNOTS - 1]);
    for m in moves {
        knots[0] = apply_move(knots[0], &m);
        for i in 1..KNOTS {
            if let Some(knot_move) = get_knot_move(knots[i - 1], knots[i]) {
                knots[i] = apply_move(knots[i], &knot_move);
            }
        }
        tail_visited.insert(knots[KNOTS - 1]);
    }
    tail_visited.len() as u32
}

fn main() {
    let input = get_input();
    let p1_total = part1(&input);
    println!("Part1 total: {}", p1_total);
    let p2_total = part2(&input);
    println!("Part2 total: {}", p2_total);
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let lines: Vec<String> = get_input();
        b.iter(|| part1(black_box(&lines)));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines: Vec<String> = get_input();
        b.iter(|| part2(black_box(&lines)));
    }
}
