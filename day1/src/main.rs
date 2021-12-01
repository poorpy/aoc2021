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

fn count_increases(measurements: Vec<u64>) -> u64 {
    let mut acc: u64 = 0;
    for (index, &item) in measurements.iter().enumerate() {
        if index + 1 < measurements.len() && item < measurements[index + 1] {
            acc += 1
        }
    }

    acc
}

fn sliding_window(measurements: Vec<u64>, window_size: usize) -> Vec<u64> {
    let mut acc: Vec<u64> = Vec::new();
    for (index, _) in measurements.iter().enumerate() {
        if index + window_size <= measurements.len() {
            acc.push(measurements[index..index + window_size].iter().sum());
        }
    }
    acc
}

fn read_measurements(filename: &str) -> Vec<u64> {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    contents
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
