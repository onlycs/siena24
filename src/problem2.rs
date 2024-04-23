use crate::common::*;
use itertools::Itertools;

pub fn concat(ints: Vec<usize>) -> Vec<usize> {
    let mut abc = 0;
    let mut de = 0;
    let mut f = 0;

    for (idx, i) in ints.iter().enumerate() {
        match idx as u32 {
            idx @ 0..=2 => abc += i * 10usize.pow(2 - idx),
            idx @ 3..=4 => de += i * 10usize.pow(4 - idx),
            5 => f = *i,
            _ => unreachable!(),
        }
    }

    vec![abc, de, f]
}

pub fn difference(ints: &Vec<usize>) -> f64 {
    ints.iter()
        .copied()
        .map(|n| n as f64)
        .map(f64::sqrt)
        .fold(None, |fold, i| match fold {
            None => Some(i),
            Some(n) => Some(n - i),
        })
        .unwrap()
}

pub fn main() {
    let num = read_line::<usize>() as f64;
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let perms = nums.into_iter().permutations(6); // python: itertools.permutations(list, num)
    let concats = perms.map(concat);
    let min = concats
        .min_by(|a, b| {
            let adiff = (num - difference(a)).abs();
            let bdiff = (num - difference(b)).abs();

            adiff.partial_cmp(&bdiff).unwrap()
        })
        .unwrap()
        .into_iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .split("")
        .collect_vec()
        .join("\n");

    println!("{}", min.trim());
}
