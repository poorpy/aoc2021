use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let commands = read_commands(filename).unwrap();

    let final_position = commands.iter().fold(
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |position, command| calculate_position(command, position),
    );

    println!("Final Position: {:?}", final_position);
    println!("Mul: {}", final_position.horizontal * final_position.depth)
}

fn calculate_position(command: &CommandPair, mut start: Position) -> Position {
    match command.direction.as_str() {
        "forward" => {
            start.horizontal += command.value;
            start.depth += command.value * start.aim;
            start
        }
        "down" => {
            start.aim += command.value;
            start
        }
        "up" => {
            start.aim -= command.value;
            start
        }
        _ => unimplemented!(),
    }
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

#[derive(Clone)]
struct CommandPair {
    direction: String,
    value: i32,
}

fn read_commands(filename: &str) -> io::Result<Vec<CommandPair>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| {
            let mut split = line.split(" ");
            let (direction, value): (&str, &str) = (split.next().unwrap(), split.next().unwrap());
            CommandPair {
                direction: direction.to_owned(),
                value: value.parse::<i32>().unwrap(),
            }
        })
        .collect())
}
