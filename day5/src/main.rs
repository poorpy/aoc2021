use std::{
    cmp::{max, min},
    collections::{hash_map::Entry, HashMap},
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let points: Vec<(u32, u32)> = read_line_segments(filename)
        .iter()
        .map(|(start, end)| points_to_indexes(start, end))
        .collect::<Vec<Vec<(u32, u32)>>>()
        .concat();

    let mut board = Board {
        vents: HashMap::new(),
    };

    board.mark(points);

    println!(
        "ovelapping: {}",
        board.vents.values().filter(|v| **v > 1).count()
    );
}

struct Board {
    vents: HashMap<(u32, u32), u32>,
}

impl Board {
    fn mark(&mut self, points: Vec<(u32, u32)>) {
        for point in points {
            match self.vents.entry(point) {
                Entry::Vacant(v) => {
                    v.insert(1);
                }
                Entry::Occupied(mut o) => {
                    o.insert(o.get() + 1);
                }
            };
        }
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

fn points_to_indexes(start: &Point, end: &Point) -> Vec<(u32, u32)> {
    let mut indexes = Vec::new();

    if (start.x == end.x) || (start.y == end.y) {
        // +1 as those ranges should include bigger number
        for x in min(start.x, end.x)..max(start.x, end.x) + 1 {
            for y in min(start.y, end.y)..max(start.y, end.y) + 1 {
                indexes.push((x, y))
            }
        }
    }

    indexes
}

fn read_line_segments(filename: &str) -> Vec<(Point, Point)> {
    let file =
        File::open(filename).expect(&format!("Something went wrong opening file {}", filename));
    io::BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| {
            let points = line
                .split_whitespace()
                .collect::<String>()
                .split("->")
                .map(|coords| {
                    let xy: Vec<u32> = coords
                        .split(",")
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect();
                    Point { x: xy[0], y: xy[1] }
                })
                .collect::<Vec<Point>>();

            (points[0], points[1])
        })
        .collect()
}
