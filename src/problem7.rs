use std::{collections::HashSet, fmt};

use crate::common::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Square {
    Given(usize),
    FollowedOnlyRoute(usize),
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(usize)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn apply_offset(&self, coord: (usize, usize)) -> Option<(usize, usize)> {
        let offset = [(-1, 0), (1, 0), (0, -1), (0, 1)][*self as usize];

        let newi = coord.0 as i32 + offset.0;
        let newj = coord.1 as i32 + offset.1;

        if newi < 0 || newj < 0 {
            return None;
        }

        Some((newi as usize, newj as usize))
    }

    fn fromoffset(a: (usize, usize), b: (usize, usize)) -> Option<Self> {
        for dir in Self::ALL {
            if dir.apply_offset(a) == Some(b) {
                return Some(dir);
            }
        }

        None
    }
}

impl Square {
    fn unknown(&self) -> bool {
        match self {
            Square::Unknown => true,
            _ => false,
        }
    }

    fn value(&self) -> Option<usize> {
        match self {
            Square::FollowedOnlyRoute(i) => Some(*i),
            Square::Given(i) => Some(*i),
            _ => None,
        }
    }
}

struct PathChooser {
    paths: Vec<Vec<Vec<(usize, usize)>>>,
    empty: usize,
}

impl PathChooser {
    pub fn new(paths: Vec<((usize, usize), Vec<Vec<Direction>>)>, empty: usize) -> Self {
        let mut paths = paths
            .into_iter()
            .map(|(src, paths)| {
                let mut listpoints = vec![vec![src]; paths.len()];

                for (i, path) in paths.into_iter().enumerate() {
                    let points = &mut listpoints[i];

                    for dir in path {
                        points.push(dir.apply_offset(*points.last().unwrap()).unwrap())
                    }
                }

                listpoints.sort_unstable_by_key(Vec::len);

                listpoints
            })
            .collect::<Vec<_>>();

        // .len() contains starting and endpoint squares. they will not be newly occupied
        let minnewsquares = paths.iter().map(|v| v[0].len() - 2).collect::<Vec<_>>();

        // get the sum of minimum lengths, excluding the one with your number on it
        let minlengths_except = |n: usize| {
            minnewsquares
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != n)
                .map(|(_, a)| *a)
                .sum::<usize>()
        };

        paths.iter_mut().enumerate().for_each(|(pathid, paths)| {
            paths.retain(|path| path.len() <= empty - minlengths_except(pathid))
        });

        Self { paths, empty }
    }

    fn backtrack<'a: 'b, 'b>(
        &'a self,
        n: usize,
        occupied: &mut HashSet<(usize, usize)>,
        collect: &mut Vec<&'b Vec<(usize, usize)>>,
        skip: &mut Vec<usize>,
        lengths: &mut Vec<usize>,
    ) {
        if skip[n] >= lengths[n] {
            if n == 0 {
                println!("NO SOLUTION");
                std::process::exit(0);
            }

            return self.backtrack(n - 1, occupied, collect, skip, lengths);
        }

        for i in n..self.paths.len() {
            if let Some(squares) = collect.get(i) {
                for sq in *squares {
                    if !occupied.remove(sq) {
                        panic!("wtf")
                    }
                }
            }

            // we want to keep skip[n], for incrementing
            // we want to keep lengths[n], because it's not changing
            if i != n {
                skip[i] = 0;
                lengths[i] = 0;
            }
        }

        collect.truncate(n);

        // choose the next path of the "bad" path
        skip[n] += 1;

        self.find(n, occupied, collect, skip, lengths);
    }

    fn find<'a: 'b, 'b>(
        &'a self,
        n: usize,
        occupied: &mut HashSet<(usize, usize)>,
        collect: &mut Vec<&'b Vec<(usize, usize)>>,
        skip: &mut Vec<usize>,
        lengths: &mut Vec<usize>,
    ) {
        if n >= lengths.len() {
            if occupied.len() != self.empty {
                self.backtrack(n, occupied, collect, skip, lengths);
            }

            return;
        }

        // filter all the paths that could possibly be chosen
        let possible = self.paths[n]
            .iter()
            .filter(|v| v.iter().all(|n| !occupied.contains(n)))
            .collect::<Vec<_>>();

        // list how many there are for backtracking
        lengths[n] = possible.len();

        // take the first
        let chosen = possible.get(skip[n]);

        // if there is one...
        if let Some(chosen) = chosen {
            for it in *chosen {
                // occupy the squares
                occupied.insert(*it);
            }

            // add it to the results
            collect.push(chosen);

            // choose the next item
            self.find(n + 1, occupied, collect, skip, lengths);
        } else {
            // backtrack!
            self.backtrack(n, occupied, collect, skip, lengths);
        }
    }

    fn choose<'a: 'b, 'b>(&'a self, collect: &mut Vec<&'b Vec<(usize, usize)>>) {
        let mut occupied = HashSet::new();
        let mut skip = vec![0; self.paths.len()];
        let mut lengths = vec![0; self.paths.len()];

        self.find(0, &mut occupied, collect, &mut skip, &mut lengths);
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Board {
    squares: Vec<Vec<Square>>,
    maxnum: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize, indexes: Vec<((usize, usize), (usize, usize))>) -> Self {
        let mut squares = vec![vec![Square::Unknown; rows]; cols];

        for (i, ((r1, c1), (r2, c2))) in indexes.iter().enumerate() {
            squares[*r1][*c1] = Square::Given(i);
            squares[*r2][*c2] = Square::Given(i);
        }

        Self {
            squares,
            maxnum: indexes.len(),
        }
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Square> {
        if i < self.squares.len() && j < self.squares[0].len() {
            Some(&mut self.squares[i][j])
        } else {
            None
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&Square> {
        if i < self.squares.len() && j < self.squares[0].len() {
            Some(&self.squares[i][j])
        } else {
            None
        }
    }

    pub fn surrounding_mut<'a>(
        &'a mut self,
        i: usize,
        j: usize,
    ) -> Vec<(&'a mut Square, (usize, usize))> {
        const MAP: [(i8, i8); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

        let mut squares = vec![];

        for (offr, offc) in MAP {
            let newr = i as i8 + offr;
            let newc = j as i8 + offc;

            if newr < 0 || newc < 0 {
                continue;
            }

            let newr = newr as usize;
            let newc = newc as usize;

            if let Some(item) = self.get_mut(newr, newc) {
                squares.push((item as *mut Square, (newr, newc)));
            }
        }

        // SAFETY: these will be different elements in the array, therefore its safe
        squares
            .into_iter()
            .map(|(sq, rc)| (unsafe { &mut *sq }, rc))
            .collect()
    }

    pub fn surrounding(&self, i: usize, j: usize) -> Vec<(&Square, (usize, usize))> {
        let mut squares = vec![];

        for dir in Direction::ALL {
            if let Some((newr, newc)) = dir.apply_offset((i, j))
                && let Some(item) = self.get(newr, newc)
            {
                squares.push((item, (newr, newc)));
            }
        }

        squares
    }

    pub fn set_oneway(&mut self, i: usize, j: usize, val: usize) -> bool {
        let surrounding = self.surrounding_mut(i, j);
        let mut surrounding = surrounding.into_iter().filter(|(sq, _)| sq.unknown());
        let nextrow;
        let nextcol;

        if let Some((next, (row, col))) = surrounding.next()
            && surrounding.next() == None
        {
            *next = Square::FollowedOnlyRoute(val);
            nextrow = row;
            nextcol = col;
        } else {
            return false;
        }

        // SAFETY: surrounding has no items (or we would have returned), therefore
        // there will not be two mutable references to the same value
        self.set_oneway(nextrow, nextcol, val);

        true
    }

    pub fn oneway_all(&mut self) -> bool {
        let mut result = false;

        for i in 0..self.squares.len() {
            for j in 0..self.squares[0].len() {
                if let Some(Square::Given(ref val)) = self.get_mut(i, j) {
                    let val = *val;

                    result |= self.set_oneway(i, j, val);
                }
            }
        }

        result
    }

    pub fn oneway_forever(&mut self) {
        while self.oneway_all() {}
    }

    pub fn pos_routes(
        &self,
        src: (usize, usize),
        traversed: Vec<(usize, usize)>,
        val: usize,
    ) -> Vec<Vec<Direction>> {
        let mut routes = vec![];
        let (fromr, fromc) = src;

        let directions = Direction::ALL.to_vec();

        let goto = directions
            .iter()
            .filter_map(|dir| {
                let Some((newr, newc)) = dir.apply_offset((fromr, fromc)) else {
                    return None;
                };

                if traversed.contains(&(newr, newc)) {
                    return None;
                }

                if self.get(newr, newc).copied() == Some(Square::Unknown) {
                    Some((newr, newc, *dir, false))
                } else if self.get(newr, newc).copied() == Some(Square::Given(val))
                    || self.get(newr, newc).copied() == Some(Square::FollowedOnlyRoute(val))
                {
                    Some((newr, newc, *dir, true))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let threads = goto.into_iter().map(|(newr, newc, offset, ended)| {
            let mut traversed = traversed.clone();

            // thread::spawn(move || {
            if ended {
                vec![vec![offset]]
            } else {
                traversed.push((newr, newc));

                self.pos_routes((newr, newc), traversed, val)
                    .into_iter()
                    .filter(|n| !n.is_empty())
                    .map(|mut n| {
                        n.insert(0, offset);
                        n
                    })
                    .collect()
            }
            // })
        });

        for t in threads {
            // let t = t.join().unwrap()
            routes.extend(t);
        }

        routes
    }

    pub fn follow_value(
        &self,
        i: usize,
        j: usize,
        prv: Option<Direction>,
        val: usize,
        traversed: &mut Vec<(usize, usize)>,
    ) -> &Square {
        traversed.push((i, j));

        let mut surrounding = self.surrounding(i, j).into_iter().filter(|(it, sq)| {
            it.value() == Some(val)
                && if let Some(prv) = prv {
                    Some(*sq) != prv.opposite().apply_offset((i, j))
                } else {
                    true
                }
        });

        if let Some((_, newcoords)) = surrounding.next() {
            let (newi, newj) = newcoords;
            let nextdir = Direction::fromoffset((i, j), newcoords).unwrap();

            self.follow_value(newi, newj, Some(nextdir), val, traversed)
        } else {
            self.get(i, j).unwrap()
        }
    }

    pub fn all_paths(&self) -> PathChooser {
        let mut follows = vec![None; self.maxnum];
        let mut paths = Vec::with_capacity(self.maxnum);

        for i in 0..self.squares.len() {
            for j in 0..self.squares[0].len() {
                if let Some(Square::Given(val)) = self.get(i, j)
                    && follows[*val] == None
                {
                    let mut traversed = vec![];
                    self.follow_value(i, j, None, *val, &mut traversed);

                    follows[*val] = Some((*traversed.last().unwrap(), traversed, *val))
                }
            }
        }

        for (src, traversed, val) in follows.into_iter().map(Option::unwrap) {
            paths.push((src, self.pos_routes(src, traversed, val)));
        }

        PathChooser::new(paths, self.empty())
    }

    fn fill_if_unfilled(&mut self, value: usize, coord: (usize, usize)) {
        let (x, y) = coord;

        match self.get_mut(x, y).unwrap() {
            Square::FollowedOnlyRoute(_) | Square::Given(_) => {}
            sq => *sq = Square::FollowedOnlyRoute(value),
        }
    }

    pub fn apply(&mut self, app: Vec<&Vec<(usize, usize)>>) {
        for (id, path) in app.into_iter().enumerate() {
            for coord in path {
                self.fill_if_unfilled(id, *coord);
            }
        }
    }

    pub fn empty(&self) -> usize {
        self.squares
            .iter()
            .map(|sqs| sqs.iter().filter(|n| n.unknown()).count())
            .sum()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.squares {
            for square in row {
                match square {
                    Square::Given(nu) => write!(f, "{}", nu + 1)?,
                    Square::FollowedOnlyRoute(nu) => {
                        write!(f, "{}", ('a' as u8 + *nu as u8) as char)?
                    }
                    Square::Unknown => write!(f, "-")?,
                }
            }

            write!(f, "\n")?;
        }

        write!(f, "")
    }
}

pub fn main() {
    let [rows, columns, nums] = read_line_vec::<usize>(" ")[..] else {
        panic!()
    };

    let mut idxs = vec![];

    for _ in 0..nums {
        let [x1, y1, x2, y2] = read_line_vec::<usize>(" ")[..] else {
            panic!()
        };

        idxs.push(((x1, y1), (x2, y2)));
    }

    let mut board = Board::new(rows, columns, idxs);
    board.oneway_forever();

    let chooser = board.all_paths();
    let mut paths = Vec::new();
    chooser.choose(&mut paths);

    board.apply(paths);

    print!("{board}");
}
