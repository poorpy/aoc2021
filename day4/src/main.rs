use std::{cmp::Ordering, env, fs::File, io, io::BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    let (numbers, mut boards) = read_numbers_and_boards(filename);
    let solutions: Vec<usize> = boards
        .iter_mut()
        .map(|mut board| solve_board(&mut board, &numbers))
        .collect();

    let optimal_index = solutions
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();

    let worst_index = solutions
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();


    println!(
        "board: {} number_called: {} mul: {}",
        boards[optimal_index].score(),
        numbers[solutions[optimal_index]],
        boards[optimal_index].score() * numbers[solutions[optimal_index]],
    );

    println!(
        "board: {} number_called: {} mul: {}",
        boards[worst_index].score(),
        numbers[solutions[worst_index]],
        boards[worst_index].score() * numbers[solutions[worst_index]],
    );
}

fn solve_board(board: &mut Board, numbers: &Vec<u32>) -> usize {
    for (index, number) in numbers.iter().enumerate() {
        if let Some(position) = board
            .numbers
            .iter()
            .position(|&item| item == (*number, false))
        {
            board.numbers[position] = (*number, true);
        }
        if board.is_solved() {
            return index;
        }
    }

    usize::MAX
}

struct Board {
    numbers: Vec<(u32, bool)>,
}

impl Board {
    fn is_solved(&self) -> bool {
        if self
            .numbers
            .chunks(5)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|pair| pair.1)
                    .fold(true, |first, second| first & second)
            })
            .fold(false, |first, second| first | second)
        {
            return true;
        }

        for i in 0..5 {
            if self
                .numbers
                .iter()
                .skip(i)
                .step_by(5)
                .map(|pair| pair.1)
                .fold(true, |first, second| first & second)
            {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|(_, marked)| !marked)
            .map(|(num, _)| num)
            .sum()
    }
}

fn from_chunk(lines: &[String]) -> Board {
    let numbers: Vec<u32> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|sub| sub.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .reduce(|mut first, second| {
            first.extend(second.into_iter());
            first
        })
        .unwrap();

    Board {
        numbers: numbers
            .into_iter()
            .zip(vec![false; 25].into_iter())
            .collect(),
    }
}

fn read_numbers_and_boards(filename: &str) -> (Vec<u32>, Vec<Board>) {
    let file =
        File::open(filename).expect(&format!("Something went wrong opening file {}", filename));
    let mut lines = io::BufReader::new(file).lines();

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let boards = lines
        .filter_map(|x| x.ok())
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<String>>()
        .chunks(5)
        .map(|chunk| from_chunk(chunk))
        .collect();

    (numbers, boards)
}
