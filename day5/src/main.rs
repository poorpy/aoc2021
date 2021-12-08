use std::{
    cmp::{max, max_by, min, min_by},
    collections::{hash_map::Entry, HashMap},
    env,
    fs::File,
    io::{self, BufRead},
    ops::Sub,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let points: Vec<(i32, i32)> = read_line_segments(filename)
        .iter()
        .map(|(start, end)| points_to_indexes(start, end))
        .collect::<Vec<Vec<(i32, i32)>>>()
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
    vents: HashMap<(i32, i32), i32>,
}

impl Board {
    fn mark(&mut self, points: Vec<(i32, i32)>) {
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
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl PartialEq for &Point {
    fn eq(&self, other: &Self) -> bool {
        self.x + self.y == other.x + other.y
    }
}

impl PartialOrd for &Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.x + self.y).cmp(&(other.x + other.y)))
    }
}

fn points_to_indexes(start: &Point, end: &Point) -> Vec<(i32, i32)> {
    let mut indexes = Vec::new();

    if (start.x == end.x) || (start.y == end.y) {
        // +1 as those ranges should include bigger number
        for x in min(start.x, end.x)..max(start.x, end.x) + 1 {
            for y in min(start.y, end.y)..max(start.y, end.y) + 1 {
                indexes.push((x, y))
            }
        }
    }

    // (0,0) to (n, n)
    let normalized: Point = max_by(start, end, |a, b| a.partial_cmp(b).unwrap()).clone()
        - min_by(start, end, |a, b| a.partial_cmp(b).unwrap()).clone();
    if normalized.x == normalized.y {
        let xs = min(start.x, end.x)..max(start.x, end.x) + 1;
        let ys = min(start.y, end.y)..max(start.y, end.y) + 1;
        for pair in xs.zip(ys) {
            indexes.push(pair)
        }
    }

    // (0, n) to (n, 0)
    if start.x + start.y == end.x + end.y {
        let xs = min(start.x, end.x)..max(start.x, end.x) + 1;
        let ys = (min(start.y, end.y)..max(start.y, end.y) + 1).rev();
        for pair in xs.zip(ys) {
            indexes.push(pair)
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
                    let xy: Vec<i32> = coords
                        .split(",")
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect();
                    Point { x: xy[0], y: xy[1] }
                })
                .collect::<Vec<Point>>();

            (points[0], points[1])
        })
        .collect()
}
