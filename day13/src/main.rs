use std::{env, fs};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    let (points, folds) = read(filename);

    let points = fold_along(points, &folds[..1]);

    println!("part1: {:?}", points.len());

    let points = fold_along(points, &folds[1..]);

    let (x, _) = points.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap();
    let (_, y) = points.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    let mut board: Vec<Vec<char>> = vec![vec!['.'; *x as usize + 1]; *y as usize + 1];

    for (x, y) in  &points {
        board[*y as usize][*x as usize] = '#' 
    }
    
    for row in board {
        println!("{:?}", row.iter().collect::<String>())
    }

}

fn fold_along(points: Vec<(u32, u32)>, folds: &[Fold]) -> Vec<(u32, u32)> {
    folds.into_iter().fold(points, |acc, fold| {
        if fold.axis == FoldAxis::X {
            fold_x(acc, fold.value)
        } else {
            fold_y(acc, fold.value)
        }
    })
}

fn fold_y(points: Vec<(u32, u32)>, value: u32) -> Vec<(u32, u32)> {
    points
        .into_iter()
        .map(|(x, y)| {
            if y < value {
                (x, y)
            } else {
                (x, (2 * value - y))
            }
        })
        .unique()
        .collect_vec()
}

fn fold_x(points: Vec<(u32, u32)>, value: u32) -> Vec<(u32, u32)> {
    points
        .into_iter()
        .map(|(x, y)| {
            if x < value {
                (x, y)
            } else {
                ((2 * value - x), y)
            }
        })
        .unique()
        .collect_vec()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FoldAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy)]
struct Fold {
    value: u32,
    axis: FoldAxis,
}

fn parse_points(points: &str) -> Vec<(u32, u32)> {
    points
        .split_whitespace()
        .filter_map(|line| {
            line.split(",")
                .filter_map(|num| num.parse::<u32>().ok())
                .next_tuple()
        })
        .collect_vec()
}

fn parse_folds(folds: &str) -> Vec<Fold> {
    folds
        .split("\n")
        .filter(|split| !split.is_empty())
        .map(|line| {
            let (i, _) = folds.char_indices().nth(11).unwrap();
            let (fold_axis, value) = &line[i..].split("=").next_tuple().unwrap();
            Fold {
                value: value.parse().unwrap(),
                axis: if fold_axis == &"x" {
                    FoldAxis::X
                } else {
                    FoldAxis::Y
                },
            }
        })
        .collect_vec()
}

fn read(filename: &str) -> (Vec<(u32, u32)>, Vec<Fold>) {
    let contents =
        fs::read_to_string(&filename).expect(&format!("couldn't read file: {}", &filename));

    let (points, folds) = contents.split("\n\n").next_tuple().unwrap();

    (parse_points(points), parse_folds(folds))
}
