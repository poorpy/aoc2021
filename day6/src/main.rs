use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let days: u64 = args[2].parse().unwrap();

    let indexes = state_into_indexes(read_initial_state(filename));

    let fish_count: u64 = (0..days)
        .fold(indexes, |state, _index| step(state))
        .iter()
        .sum();

    println!("{}", fish_count);
}

fn step(mut state: Vec<u64>) -> Vec<u64> {
    state.rotate_left(1);
    state[6] += state[8];
    state
}

fn state_into_indexes(state: Vec<u64>) -> Vec<u64> {
    let mut vec = vec![0; 9];

    for i in 0..9 {
        vec[i] = state.iter().filter(|&&x| x == i as u64).count() as u64;
    }

    vec
}

fn read_initial_state(filename: &str) -> Vec<u64> {
    fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .split(",")
        .map(|f| f.parse::<u64>().unwrap())
        .collect()
}
