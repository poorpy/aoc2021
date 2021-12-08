use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    let mut state = read_initial_state(filename);
    state.sort();

    let final_state: Vec<u32> = (0..80).fold(state, |state, _index| step(state));
    
    println!("{:?}", final_state);
}

fn step(state: Vec<u32>) -> Vec<u32> {
    let new_fish = state.iter().filter(|&&x| x == 0).count();
    let mut pending: Vec<u32> = state
        .into_iter()
        .filter(|&x| x > 0)
        .map(|x| x - 1)
        .collect();
    pending.extend(vec![vec![6; new_fish], vec![8; new_fish]].concat());
    pending
}

fn read_initial_state(filename: &str) -> Vec<u32> {
    fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .split(",")
        .map(|f| f.parse::<u32>().unwrap())
        .collect()
}
