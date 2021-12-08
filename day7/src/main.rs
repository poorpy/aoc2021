use std::{
    cmp::{max, min},
    env,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let mut crabs = read_positions(filename);
    crabs.sort();
    let median = median_value(&crabs);
    let fuel_used: i32 = crabs
        .iter()
        .map(|&crab| distance_to_move(crab, median))
        .sum();
    println!("fuel used: {}", fuel_used);

    let fuel_used = lowest_fuel(&crabs);
    println!("fuel used: {}", fuel_used);
}

fn distance_to_move(crab: i32, median: i32) -> i32 {
    max(crab, median) - min(crab, median)
}

fn lowest_fuel(crabs: &Vec<i32>) -> i32 {
    let sum: i32 = crabs.iter().sum();
    let first_pos = sum / crabs.len() as i32;
    let second_pos = first_pos + 1;
    let first_sum = crabs
        .iter()
        .map(|crab| (((first_pos - crab).abs() + 1) * (first_pos - crab).abs()) / 2)
        .sum();
    let second_sum = crabs.iter()
        .map(|crab| (((second_pos - crab).abs() + 1) * (second_pos - crab).abs()) / 2)
        .sum();
    min(first_sum, second_sum)
}

fn median_value(crabs: &Vec<i32>) -> i32 {
    crabs[crabs.len() / 2]
}

fn read_positions(filename: &str) -> Vec<i32> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .split(",")
        .map(|pos| pos.parse::<i32>().unwrap())
        .collect()
}
