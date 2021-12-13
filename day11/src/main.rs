use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let octopuses = read(filename);

    let (mut octopuses, flashes, _) = (0..100).fold((octopuses, 0, false), |(octo, count_acc, _), _| {
        let (octo, count, _) = step(octo);
        (octo, count + count_acc, false)
    });

    println!("flashes: {}", flashes);

    let mut counter = 100;
    loop {
        counter += 1;
        let (_octopuses, _, all) = step(octopuses);
        octopuses = _octopuses;
        if all {
            break;
        }
    }

    println!("step: {}", counter)
}

fn step(mut board: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, usize, bool) {
    let xs = 0..board[0].len();
    let ys = 0..board.len();
    let cartesian = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));

    let mut to_flash: Vec<(usize, usize)> = Vec::new();
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    for (x, y) in cartesian {
        board[y][x] += 1;

        if board[y][x] > 9 {
            to_flash.push((x, y));
        }
    }

    while let Some((x, y)) = to_flash.pop() {
        let x = x as isize;
        let y = y as isize;
        flash(&mut board, &mut to_flash, &mut flashed, x, y);
    }

    let all_flashed = flashed.len() == board.len() * board[0].len();
    (board, flashed.len(), all_flashed)
}

fn flash(
    board: &mut Vec<Vec<u32>>,
    to_visit: &mut Vec<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
    x: isize,
    y: isize,
) {
    let xs = -1..2;
    let ys = -1..2;
    let offsets = ys
        .flat_map(|y| xs.clone().map(move |x| (x, y)))
        .filter(|&pair| pair != (0, 0));

    let x_y = (x as usize, y as usize);

    if visited.contains(&x_y) {
        return;
    }

    board[y as usize][x as usize] = 0;
    visited.insert(x_y);

    for (x_o, y_o) in offsets {
        let y_u = usize::try_from(y + y_o);
        let x_u = usize::try_from(x + x_o);

        if !y_u.is_ok() || !x_u.is_ok() {
            continue;
        }

        let y_u = y_u.unwrap();
        let x_u = x_u.unwrap();

        if y_u < board.len() && x_u < board[0].len() {
            if !visited.contains(&(x_u, y_u)) {
                board[y_u][x_u] += 1;
            }
            if board[y_u][x_u] > 9 {
                to_visit.push((x_u, y_u));
            }
        }
    }
}

fn read(filename: &str) -> Vec<Vec<u32>> {
    let file = File::open(filename).expect(&format!("couldn't open file {}", filename));
    BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
