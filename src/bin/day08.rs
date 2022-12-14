#![feature(test)]
extern crate test;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::{iproduct, Itertools};

const DAY: &str = "08";

fn get_input() -> impl Iterator<Item = String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn get_grid(lines: &Vec<String>) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        let mut current_line = Vec::new();
        for char in line.chars() {
            let v = char.to_string().parse::<u8>().expect("Invalid height");
            current_line.push(v);
        }
        grid.push(current_line);
    }
    grid
}

fn part1(lines: &Vec<String>) -> u32 {
    let grid = get_grid(lines);
    let mut visible_set: HashSet<(u32, u32)> = HashSet::new();
    let n = grid.len();
    let m = grid[0].len();
    //Line by line
    for i in 1..n - 1 {
        let line = &grid[i];
        let mut line_max = line[0];
        for j in 1..m - 1 {
            if line[j] > line_max {
                line_max = line[j];
                visible_set.insert((i as u32, j as u32));
            }
        }
        line_max = line[n - 1];
        for j in 2..n - 1 {
            if line[n - j] > line_max {
                line_max = line[n - j];
                visible_set.insert((i as u32, (n - j) as u32));
            }
        }
    }
    for j in 1..m - 1 {
        let mut col_max = grid[0][j];
        for i in 1..n - 1 {
            if grid[i][j] > col_max {
                col_max = grid[i][j];
                visible_set.insert((i as u32, j as u32));
            }
        }
        let mut col_max = grid[n - 1][j];
        for i in 2..n - 1 {
            if grid[n - i][j] > col_max {
                col_max = grid[n - i][j];
                visible_set.insert(((n - i) as u32, j as u32));
            }
        }
    }
    let edge_visible_trees = (2 * n + 2 * m - 4) as u32;
    let inner_visible_trees = visible_set.len() as u32;

    edge_visible_trees + inner_visible_trees
}
fn part2(lines: &Vec<String>) -> u32 {
    let grid = get_grid(lines);
    let n = grid.len();
    let m = grid[0].len();
    let get_scenic_score = |i: usize, j: usize| -> u32 {
        if i == 0 || i == n - 1 || j == 0 || j == m - 1 {
            return 0;
        }
        let height = grid[i][j];

        let bottom_visibility = ((i + 1..n).take_while(|k| height > grid[*k][j]).count() + 1)
            .clamp(0, (i + 1..n).count());
        let top_visibility = ((0..=i - 1)
            .rev()
            .take_while(|k| height > grid[*k][j])
            .count()
            + 1)
        .clamp(0, (0..=i - 1).count());
        let right_visibility = ((j + 1..m).take_while(|k| height > grid[i][*k]).count() + 1)
            .clamp(0, (j + 1..m).count());
        let left_visibility = ((0..=j - 1)
            .rev()
            .take_while(|k| height > grid[i][*k])
            .count()
            + 1)
        .clamp(0, (0..=j - 1).count());

        (bottom_visibility * top_visibility * right_visibility * left_visibility) as u32
    };
    iproduct!(1..n - 1, 1..m - 1)
        .map(|(i, j)| get_scenic_score(i, j))
        .max()
        .unwrap()
}

fn main() {
    let input = get_input().collect_vec();
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
        let lines: Vec<String> = get_input().collect();
        b.iter(|| part1(black_box(&lines)));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines: Vec<String> = get_input().collect();
        b.iter(|| part2(black_box(&lines)));
    }
}
