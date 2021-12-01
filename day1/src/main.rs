use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let window_size: usize = args[2].parse().unwrap();

    println!(
        "increases: {}",
        count_increases(sliding_window(read_measurements(filename), window_size))
    )
}

fn count_increases(measurements: Vec<i32>) -> i32 {
    measurements
        .iter()
        .zip(measurements[1..].iter())
        .map(|(first, second)| if first < second { 1 } else { 0 })
        .sum()
}

fn sliding_window(measurements: Vec<i32>, window_size: usize) -> Vec<i32> {
    measurements
        .windows(window_size)
        .map(|x| x.iter().sum())
        .collect()
}

fn read_measurements(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    contents
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
