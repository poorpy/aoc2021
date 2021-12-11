use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let input = read_subystems(filename);
    let points: u64 = input
        .iter()
        .map(|l| first_illegal(l))
        .map(|i| illegal_to_points(i))
        .sum();

    println!("points: {}", points);

    let mut missig_points: Vec<u64> = input
        .iter()
        .map(|l| (l, first_illegal(l)))
        .filter_map(|(l, i)| if i == 'a' { Some(l) } else { None })
        .map(|l| {
            missing_parens(l)
                .iter()
                .fold(0, |acc, points| acc * 5 + points)
        }).collect();

    missig_points.sort();

    println!("missig_points: {}", missig_points[missig_points.len() / 2])

}

fn paren_to_points(paren: char) -> u64 {
    match paren {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn missing_parens(line: &String) -> Vec<u64> {
    let mut stack: Vec<char> = Vec::new();
    for paren in line.chars() {
        match paren {
            '(' | '[' | '{' | '<' => {
                stack.push(paren);
            }
            ')' | ']' | '}' | '>' => {
                stack.pop();
            }
            _ => {
                continue;
            }
        }
    }
    stack.iter().rev().map(|p| paren_to_points(*p)).collect()
}


fn first_illegal(line: &String) -> char {
    let mut opening: Vec<char> = Vec::new();

    for item in line.chars() {
        match item {
            '(' | '[' | '{' | '<' => {
                opening.push(item);
            }
            ')' | ']' | '}' | '>' => {
                if let Some(o) = opening.pop() {
                    if o as u64 + 1 != item as u64 && o as u64 + 2 != item as u64 {
                        return item;
                    }
                } else {
                    return item;
                }
            }

            _ => {
                continue;
            }
        }
    }
    'a'
}

fn illegal_to_points(illegal: char) -> u64 {
    match illegal {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn read_subystems(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect(&format!("couldn't open file {}", filename));

    io::BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect()
}
