use core::fmt;
use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::common::*;

type Square = Option<usize>;
type Path = Vec<Coordinate>;

static ROWS: AtomicUsize = AtomicUsize::new(0);
static COLS: AtomicUsize = AtomicUsize::new(0);

fn rows() -> usize {
    ROWS.load(Ordering::Relaxed)
}

fn cols() -> usize {
    COLS.load(Ordering::Relaxed)
}

fn num_squares() -> usize {
    rows() * cols()
}

#[derive(Clone, Debug)]
struct Board(Vec<Vec<Square>>);

impl Board {
    fn apply_path(&mut self, p: Path, i: usize) {
        for coord in p {
            self[coord] = Some(i);
        }
    }

    fn apply_paths(&mut self, paths: Vec<Path>) {
        for (i, p) in paths.into_iter().enumerate() {
            self.apply_path(p, i);
        }
    }

    fn to_string(&self, given: Vec<(Coordinate, Coordinate)>) -> String {
        let mut use_nums = given
            .iter()
            .flat_map(|(a, b)| [*a, *b])
            .collect::<HashSet<_>>();

        let mut s = String::new();

        for i in 0..rows() {
            for j in 0..cols() {
                let coord = Coordinate::new(i, j);

                match self[coord] {
                    Some(n) if use_nums.contains(&coord) => {
                        s.push((b'1' + n as u8) as char);
                    }
                    Some(n) => {
                        s.push((b'a' + n as u8) as char);
                        use_nums.insert(coord);
                    }
                    None => {
                        s.push('.');
                    }
                }

                s.push(' ');
            }

            s.push('\n');
        }

        s
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn min_distance(&self, other: Self) -> usize {
        (self.x as isize - other.x as isize).abs() as usize
            + (self.y as isize - other.y as isize).abs() as usize
    }

    fn offset(&self, d: Direction) -> Option<Self> {
        // ensure we dont exceed rows and cols
        match d {
            Direction::Up if self.y > 0 => Some(Self::new(self.x, self.y - 1)),
            Direction::Down if self.y + 1 < rows() => Some(Self::new(self.x, self.y + 1)),
            Direction::Left if self.x > 0 => Some(Self::new(self.x - 1, self.y)),
            Direction::Right if self.x + 1 < cols() => Some(Self::new(self.x + 1, self.y)),
            _ => None,
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Index<Coordinate> for Board {
    type Output = Square;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl IndexMut<Coordinate> for Board {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}

#[derive(Clone, Debug)]
struct PathGenerator {
    paths: Vec<Vec<Path>>,
    targets: Vec<(Coordinate, Coordinate)>,
}

impl PathGenerator {
    fn new(givens: Vec<(Coordinate, Coordinate)>) -> Self {
        let mut targets = vec![(Coordinate::origin(), Coordinate::origin()); givens.len()];

        for (i, (start, end)) in givens.into_iter().enumerate() {
            targets[i] = (start, end);
        }

        Self {
            paths: vec![vec![]; targets.len()],
            targets,
        }
    }

    // the maximum length of the path that can be generated
    // for any given target
    fn max_path_len(&self, n: usize) -> usize {
        num_squares()
            - self
                .targets
                .iter()
                .enumerate()
                .map(|(i, (a, b))| {
                    if i < n {
                        self.paths[i][0].len()
                    } else if i != n {
                        a.min_distance(*b)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
    }

    fn generate(&mut self, n: usize) {
        let (src, dest) = self.targets[n];
        let max_len = self.max_path_len(n);

        let mut traceback = vec![Direction::ALL.into_iter()];
        let mut visited =
            HashSet::<Coordinate>::from_iter(self.targets.iter().flat_map(|(a, b)| [*a, *b]));
        let mut path = vec![src];

        visited.remove(&dest);

        while let Some(coord) = path.last()
            && let Some(directions) = traceback.last_mut()
        {
            // if too long, backtrack
            if path.len() >= max_len {
                traceback.pop();
                visited.remove(&path.pop().unwrap());
                continue;
            }

            // get the next coordinate
            let coord = directions.find_map(|direction| {
                coord
                    .offset(direction)
                    .filter(|coordinate| !visited.contains(coordinate))
            });

            // if no more coords, backtrack
            let Some(coord) = coord else {
                traceback.pop();
                visited.remove(&path.pop().unwrap());
                continue;
            };

            // if we reached the target, add the path
            // there still may be more available next
            // directions, so don't backtrack
            if coord == dest {
                self.paths[n].push({
                    let mut path = path.clone();
                    path.push(coord);
                    path
                });

                continue;
            }

            // add the coord to the path
            // and set up for next iter
            path.push(coord);
            visited.insert(coord);
            traceback.push(Direction::ALL.into_iter());
        }

        self.paths[n].sort_unstable_by_key(Vec::len);
    }

    fn generate_all(&mut self) {
        for i in 0..self.targets.len() {
            self.generate(i);
        }
    }

    fn collect(self) -> Vec<Vec<Path>> {
        self.paths
    }
}

#[derive(Clone, Debug)]
struct PathChooser {
    paths: Vec<Vec<Path>>,
    chosen: Vec<usize>,
    visited: HashSet<Coordinate>,
}

impl PathChooser {
    fn new(paths: Vec<Vec<Path>>) -> Self {
        Self {
            chosen: vec![0; paths.len()],
            visited: HashSet::new(),
            paths,
        }
    }

    fn backtrack(&mut self, n: usize) {
        for coord in &self.paths[n][self.chosen[n]] {
            self.visited.remove(coord);
        }

        if n + 1 < self.chosen.len() {
            self.chosen[n + 1] = 0;
        }

        self.chosen[n] += 1;

        self.choose(n);
    }

    fn choose(&mut self, n: usize) {
        if n == self.paths.len() {
            if self.visited.len() != num_squares() {
                // we have not filled the grid,
                // therefore this is not a valid solution
                self.backtrack(n - 1);
            }

            return;
        }

        while self.chosen[n] < self.paths[n].len() {
            let path = &self.paths[n][self.chosen[n]];

            if path.iter().all(|n| !self.visited.contains(n)) {
                for &coord in path {
                    self.visited.insert(coord);
                }

                return self.choose(n + 1);
            } else {
                self.chosen[n] += 1;
                continue;
            }
        }

        if n == 0 {
            println!("NO SOLUTION");
            std::process::exit(0);
        }

        self.backtrack(n - 1);
    }

    fn trim(&mut self) {
        let min_lengths = self
            .paths
            .iter()
            .map(|paths| paths[0].len())
            .collect::<Vec<_>>();

        let total_min_length = min_lengths.iter().sum::<usize>();
        let num_squares = num_squares();

        for (i, paths) in self.paths.iter_mut().enumerate() {
            let max_length = num_squares - (total_min_length - min_lengths[i]);
            paths.retain(|path| path.len() <= max_length);
        }
    }

    fn choose_all(&mut self) {
        self.trim();
        self.choose(0);
    }

    fn collect(self) -> Vec<Path> {
        self.paths
            .into_iter()
            .zip(self.chosen)
            .map(|(mut paths, chosen)| paths.swap_remove(chosen))
            .collect()
    }
}

pub fn main() {
    let [rows, cols, nums] = read_line_vec(" ")[..] else {
        panic!()
    };

    ROWS.store(rows, Ordering::Relaxed);
    COLS.store(cols, Ordering::Relaxed);

    let mut coordinates = vec![];

    for _ in 0..nums {
        let [x1, y1, x2, y2] = read_line_vec(" ")[..] else {
            panic!()
        };

        coordinates.push((Coordinate::new(x1, y1), Coordinate::new(x2, y2)));
    }

    let mut generator = PathGenerator::new(coordinates.clone());
    let mut board = Board(vec![vec![None; cols]; rows]);
    generator.generate_all();

    let paths = generator.collect();
    let mut chooser = PathChooser::new(paths);
    chooser.choose_all();

    let paths = chooser.collect();
    board.apply_paths(paths);

    println!("{}", board.to_string(coordinates));
}
