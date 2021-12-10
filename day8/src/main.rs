use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

type PatternToDigit = HashMap<String, String>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let readings = readings(filename);
    let sum: u32 = readings
        .into_iter()
        .map(|(input, output)| {
            let decode_map = pattern_map(input);
            decode(output, decode_map)
        })
        .sum();

    println!("sum: {}", sum);

    // part 1 below
    // let mapped = readings[0]
    //     .1
    //     .iter()
    //     .map(|section| map_chars(section, &char_map))
    //     .collect::<Vec<String>>();

    // let count = readings
    //     .into_iter()
    //     .map(|(_, x)| x)
    //     .concat()
    //     .iter()
    //     .filter(|s| match s.len() {
    //         2 | 4 | 3 | 7 => true,
    //         _ => false,
    //     })
    //     .count();

    // println!("count: {}", count);
}

fn decode(input: Vec<String>, map: PatternToDigit) -> u32 {
    let sorted: Vec<String> = input
        .into_iter()
        .map(|s| s.chars().sorted().collect::<String>())
        .collect();
    sorted
        .iter()
        .map(|s| map[s].clone())
        .collect_vec()
        .join("")
        .parse::<u32>()
        .unwrap()
}

fn pattern_map(mut input: Vec<String>) -> PatternToDigit {
    let mut ret: PatternToDigit = HashMap::new();
    input.sort_by(|a, b| a.len().cmp(&b.len()));
    let sorted: Vec<String> = input
        .into_iter()
        .map(|s| s.chars().sorted().collect::<String>())
        .collect();
    // 1, 4, 7, 8 have distinct len
    ret.insert(sorted[0].to_owned(), "1".to_owned());
    ret.insert(sorted[1].to_owned(), "7".to_owned());
    ret.insert(sorted[2].to_owned(), "4".to_owned());
    ret.insert(sorted[9].to_owned(), "8".to_owned());

    // six is only 6 seg number not containing all elements of "1"
    let six: String = sorted
        .iter()
        .filter(|s| s.len() == 6)
        .filter(|s| !sorted[0].chars().all(|c| s.contains(c)))
        .next()
        .unwrap()
        .clone();
    ret.insert(six.clone(), "6".to_owned());

    let one_segments: HashSet<char> = sorted[0].chars().collect();
    let four_segments: HashSet<char> = sorted[2].chars().collect();
    let b_and_d: HashSet<char> = four_segments.difference(&one_segments).cloned().collect();
    let b: char = sorted
        .iter()
        .filter(|s| s.len() == 6)
        .map(|s| s.chars().collect::<HashSet<char>>())
        .fold(b_and_d.clone(), |acc, set| {
            acc.intersection(&set).cloned().collect::<HashSet<char>>()
        })
        .into_iter()
        .next()
        .unwrap();
    let d: char = b_and_d
        .iter()
        .filter(|&&item| item != b)
        .next()
        .unwrap()
        .clone();

    let nine: String = sorted
        .iter()
        .filter(|s| s.len() == 6)
        .filter(|s| s.clone() != &six)
        .filter(|s| s.contains(d))
        .next()
        .unwrap()
        .clone();
    ret.insert(nine.clone(), "9".to_owned());

    // zero doesn't contain d -> middle horizontal dash
    let zero: String = sorted
        .iter()
        .filter(|s| s.len() == 6)
        .filter(|s| s.clone() != &six)
        .filter(|s| !s.contains(d))
        .next()
        .unwrap()
        .clone();
    ret.insert(zero.clone(), "0".to_owned());

    // five is the only 5 seg symbol containing b
    let five: String = sorted
        .iter()
        .filter(|s| s.len() == 5)
        .filter(|s| s.contains(b))
        .next()
        .unwrap()
        .clone();
    ret.insert(five.clone(), "5".to_owned());

    // five is the only 5 seg symbol containing all segmets of 1
    let three: String = sorted
        .iter()
        .filter(|s| s.len() == 5)
        .filter(|s| {
            let mut chars = sorted[0].chars();
            let first_char: char = chars.next().unwrap();
            let second_char: char = chars.next().unwrap();

            s.contains(first_char) && s.contains(second_char)
        })
        .next()
        .unwrap()
        .clone();

    ret.insert(three.clone(), "3".to_owned());

    let two: String = sorted
        .iter()
        .filter(|s| s.len() == 5)
        .filter(|s| s.clone() != &five && s.clone() != &three)
        .next()
        .unwrap()
        .clone();
    ret.insert(two, "2".to_owned());
    ret
}

fn readings(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open(filename).expect(&format!("Couldn't open file: {}", filename));

    io::BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| {
            let (input, output) = line.split("|").next_tuple().unwrap();
            (
                input.split_whitespace().map(|s| s.to_owned()).collect(),
                output.split_whitespace().map(|s| s.to_owned()).collect(),
            )
        })
        .collect()
}
