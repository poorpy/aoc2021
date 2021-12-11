use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let (input, line_length) = read_input(filename);
    let low_points = low_points(&input, line_length);
    let sum = risk_level_sum(&input, &low_points);
    println!("sum: {}", sum);

    let mut basins: Vec<usize> = low_points
        .iter()
        .map(|point| {
            let mut to_visit: Vec<usize> = vec![*point];
            let mut visited: HashSet<usize> = HashSet::new();
            let size = basin_size(&mut to_visit, &mut visited, &input, line_length);
            size
        })
        .collect();

    basins.sort();

    let biggest = basins.iter().rev().take(3).fold(1, |acc, item| acc * item);
    println!("ans: {}", biggest);
}

fn risk_level_sum(input: &Vec<u32>, low_points: &Vec<usize>) -> u32 {
    low_points.into_iter().map(|i| input[*i] + 1).sum()
}

fn basin_size(
    to_visit: &mut Vec<usize>,
    visited: &mut HashSet<usize>,
    floor_map: &Vec<u32>,
    line_length: usize,
) -> usize {
    if to_visit.is_empty() {
     return visited.len();
    }

    let low_point = to_visit.pop().unwrap(); 

    visited.insert(low_point);

    // top
    if low_point >= line_length
        && floor_map[low_point - line_length] != 9
            && !visited.contains(&(low_point - line_length))
    {
        to_visit.push(low_point - line_length);
    }

    // bottom
    let bottom = low_point + line_length;
    if bottom < floor_map.len() && floor_map[bottom] != 9 && !visited.contains(&bottom) {
        to_visit.push(bottom);
    }

    //left
    if low_point % line_length >= 1
        && floor_map[low_point - 1] != 9
            && !visited.contains(&(low_point - 1))
    {
        to_visit.push(low_point - 1);
    }

    //right
    let right = low_point + 1;
    if right < floor_map.len()
        && (low_point % line_length) + 1 < line_length
            && floor_map[right] != 9
            && !visited.contains(&right)
    {
        to_visit.push(low_point + 1);
    }

    basin_size(to_visit, visited, floor_map, line_length)
}

fn low_points(input: &Vec<u32>, line_length: usize) -> Vec<usize> {
    input
        .iter()
        .enumerate()
        .filter(|(i, x)| {
            let top = if i >= &line_length {
                input[(i - line_length)]
            } else {
                u32::MAX
            };
            let bottom = if i + line_length < input.len() {
                input[i + line_length]
            } else {
                u32::MAX
            };
            let left = if i % line_length >= 1 {
                input[i - 1]
            } else {
                u32::MAX
            };
            let right = if i + 1 < input.len() && (i % line_length) + 1 < line_length {
                input[i + 1]
            } else {
                u32::MAX
            };

            *x < &top && *x < &bottom && *x < &left && *x < &right
        })
        .map(|(i, _)| i)
        .collect()
}

fn read_input(filename: &str) -> (Vec<u32>, usize) {
    let file = File::open(filename).expect(&format!("Couldn't open file: {}", filename));

    let floor_map: Vec<Vec<u32>> = io::BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).expect(&format!("incorrect digit {}", c)))
                .collect()
        })
        .collect();
    let line_length = floor_map[0].len();

    (floor_map.concat(), line_length)
}
