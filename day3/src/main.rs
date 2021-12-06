use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let line_length: usize = (&args[2]).parse().unwrap();
    let report = read_diagnostic_report(filename);

    let (gamma, epsilon) = gamma_and_epsilon_rates(&report, line_length);
    println!(
        "gamma: {} epsilon: {} mul: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let oxygen = oxygen_rating(
        report
            .chars()
            .collect::<Vec<char>>()
            .chunks(line_length)
            .collect(),
        line_length,
        0,
    );

    let co2 = co2_rating(
        report
            .chars()
            .collect::<Vec<char>>()
            .chunks(line_length)
            .collect(),
        line_length,
        0,
    );
    println!("oxygen: {} co2: {} mul: {}", oxygen, co2, oxygen*co2)
}

fn gamma_and_epsilon_rates(report: &String, line_length: usize) -> (u32, u32) {
    let mut gamma_rate: String = String::new();
    for i in 0..line_length {
        gamma_rate.push(most_common_at_nth_pos(&report, line_length, i));
    }

    let epsilon_rate: String = gamma_rate
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect();

    (
        u32::from_str_radix(&gamma_rate, 2).unwrap(),
        u32::from_str_radix(&epsilon_rate, 2).unwrap(),
    )
}

fn oxygen_rating(report: Vec<&[char]>, line_length: usize, n: usize) -> u32 {
    let most_common = most_common_at_nth_pos(&report.concat().iter().collect(), line_length, n);
    let matching: Vec<&[char]> = report
        .into_iter()
        .filter(|chunk| chunk[n] == most_common)
        .collect();

    if matching.len() == 1 {
        return u32::from_str_radix(matching[0].iter().collect::<String>().as_str(), 2).unwrap();
    }

    oxygen_rating(matching, line_length, n + 1)
}

fn co2_rating(report: Vec<&[char]>, line_length: usize, n: usize) -> u32 {
    let most_common = most_common_at_nth_pos(&report.concat().iter().collect(), line_length, n);
    let matching: Vec<&[char]> = report
        .into_iter()
        .filter(|chunk| chunk[n] != most_common)
        .collect();

    if matching.len() == 1 {
        return u32::from_str_radix(matching[0].iter().collect::<String>().as_str(), 2).unwrap();
    }

    co2_rating(matching, line_length, n + 1)
}

fn most_common_at_nth_pos(report: &String, line_length: usize, n: usize) -> char {
    let bits_at_nth_pos: String = report.chars().skip(n).step_by(line_length).collect();

    if bits_at_nth_pos.matches("0").count() > bits_at_nth_pos.matches("1").count() {
        return '0';
    }

    '1'
}

fn read_diagnostic_report(filename: &str) -> String {
    let mut contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    contents.retain(|c| !c.is_whitespace());

    contents
}
