use std::{
    fmt,
    io::{self, BufRead},
    str::FromStr,
};

pub fn read_line<T: FromStr>() -> T
where
    <T as FromStr>::Err: fmt::Debug,
{
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();

    line.trim().parse().unwrap()
}

pub fn read_line_vec<T: FromStr>(splitpat: &'static str) -> Vec<T>
where
    <T as FromStr>::Err: fmt::Debug,
{
    let line: String = read_line();

    line.split(splitpat)
        .filter(|n| !n.is_empty())
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn n_inputs<T: FromStr>(n: usize) -> Vec<T>
where
    <T as FromStr>::Err: fmt::Debug,
{
    let mut v = vec![];

    for _ in 0..n {
        v.push(read_line());
    }

    v
}
