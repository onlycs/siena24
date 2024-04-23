use crate::common::*;
use itertools::Itertools;

fn loser(v: &mut Vec<Vec<usize>>, votes: &Vec<usize>) -> usize {
    let mut firstplacevotes = vec![usize::MAX; 4];

    for (idx, vote) in votes.iter().enumerate() {
        if firstplacevotes[v[idx][0]] == usize::MAX {
            firstplacevotes[v[idx][0]] = 0;
        }

        firstplacevotes[v[idx][0]] += *vote;
    }

    let min = firstplacevotes
        .iter()
        .enumerate()
        .min_by_key(|(_, b)| **b)
        .unwrap()
        .0;

    v.iter_mut().for_each(|v| {
        v.remove(v.iter().position(|n| *n == min).unwrap());
    });

    votes.iter().copied().sum::<usize>() - firstplacevotes[min]
}

pub fn main() {
    let mut v = vec![0, 1, 2, 3].into_iter().permutations(4).collect_vec();
    let votes = read_line_vec::<usize>(" ");

    loser(&mut v, &votes);
    loser(&mut v, &votes);
    let val = loser(&mut v, &votes);

    println!("{}", ('A' as u8 + v[0][0] as u8) as char);
    println!("{val}");
}
