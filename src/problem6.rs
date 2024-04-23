use crate::common::*;
use std::{collections::LinkedList, iter};

fn push_sorted(list: &mut LinkedList<usize>, it: usize) {
    let mut j = list.len();

    for (idx, i) in list.iter().enumerate() {
        if *i > it {
            j = idx;
            break;
        }
    }

    let mut tail = list.split_off(j);
    list.push_back(it);
    list.append(&mut tail);
}

fn n_in_a_row(list: &LinkedList<usize>, n: usize) -> Option<usize> {
    let mut current = 0;
    let mut inarow = 0;
    let mut ifirst = 0;

    for (idx, it) in list.iter().enumerate() {
        if inarow == n {
            return Some(ifirst);
        }

        if *it == current + 1 {
            inarow += 1;
        } else {
            inarow = 0;
            ifirst = idx;
        }

        current = *it;
    }

    None
}

fn _gcd(a: usize, b: usize) -> usize {
    let mut gcd = 1;

    for i in 2..=(a.min(b) as f64).sqrt().ceil() as usize {
        if a % i == 0 && b % i == 0 {
            gcd = i;
        }
    }

    gcd
}

fn gcd(n: &Vec<usize>) -> usize {
    let mut g = _gcd(n[0], n[1]);

    for i in n.iter().skip(2) {
        g = _gcd(g, *i);
    }

    g
}

pub fn main() {
    let mut list = LinkedList::from_iter(iter::once(0));
    let input = n_inputs::<usize>(read_line());

    if gcd(&input) != 1 {
        println!("INFINITE");
        return;
    }

    if input.contains(&1) {
        println!("ALL");
        return;
    }

    if input.len() == 1 {
        println!("INFINITE");
        return;
    }

    let n;

    loop {
        for i in list.clone() {
            for j in &input {
                let n = i + *j;
                if !list.contains(&n) {
                    push_sorted(&mut list, n);
                }
            }
        }

        if let Some(i) = n_in_a_row(&list, input[0]) {
            n = i;
            break;
        }
    }

    println!("{}", list.iter().skip(n).next().unwrap() - 1);
}
