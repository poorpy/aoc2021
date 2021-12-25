use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let mut board = read(filename);
    
    let mut counter = 0;
    loop {

        let moved = step(&mut board);
        counter += 1;

        if !moved {
            break;
        }
    }

    println!("moves: {}", counter)

}

type Coords = (usize, usize); // y, x

fn step(board: &mut Board) -> bool {
    let mut swap_stack: Vec<(Coords, Coords)> = Vec::new();
    let mut moved = false;

    step_east(&board, &mut &mut swap_stack);
    moved = moved || swap_stack.len() > 0;
    swap(board, &mut swap_stack);

    step_south(&board, &mut swap_stack);
    moved = moved || swap_stack.len() > 0;
    swap(board, &mut swap_stack);

    moved
}

fn swap(board: &mut Board, swap_stack: &mut Vec<(Coords, Coords)>) {
    while let Some(((y1, x1), (y2, x2))) = swap_stack.pop() {
        let mem = board[y1][x1];
        board[y1][x1] = board[y2][x2];
        board[y2][x2] = mem;
    }
}


fn step_east(board: &Board, swap_stack: &mut Vec<(Coords, Coords)>) {
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] != '>' {
                continue;
            }
            let next = (x + 1) % board[y].len();
            if board[y][next] == '.' {
                swap_stack.push(((y, x), (y, next)));
            }
        }
    }
}

fn step_south(board: &Board, swap_stack: &mut Vec<(Coords, Coords)>) {
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] != 'v' {
                continue;
            }
            let next = (y + 1) % board.len();
            if board[next][x] == '.' {
                swap_stack.push(((y, x), (next, x)));
            }
        }
    }
}

type Board = Vec<Vec<char>>;

fn read(filename: &str) -> Board {
    let file = File::open(filename).expect(&format!("couldn't open file {}", filename));
    BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
