use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let line_length: usize = (&args[2]).parse().unwrap();
    let report = read_diagnostic_report(filename);

    let (gamma, epsilon) = gamma_and_epsilon_rates(report, line_length);
    println!(
        "gamma: {} epsilon: {} mul: {}",
        gamma,
        epsilon,
        gamma * epsilon
    )
}

fn gamma_and_epsilon_rates(report: String, line_length: usize) -> (u32, u32) {
    let mut gamma_rate: String = String::new();
    for i in 0..line_length {
        let bits_at_nth_pos: String = report.chars().skip(i).step_by(line_length).collect();

        if bits_at_nth_pos.matches("0").count() > bits_at_nth_pos.matches("1").count() {
            gamma_rate.push('0')
        } else {
            gamma_rate.push('1')
        }
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

fn read_diagnostic_report(filename: &str) -> String {
    let mut contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    contents.retain(|c| !c.is_whitespace());

    contents
}
