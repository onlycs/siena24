use crate::common::*;

pub fn main() {
    let input = read_line::<usize>();

    let mut solomon = vec![1, 2, 2];
    let mut wants = 3;

    while solomon.len() <= input {
        let numwants = solomon[wants - 1];
        solomon.extend_from_slice(&vec![wants; numwants]);
        wants += 1;
    }

    println!("{}", solomon[input - 1]);
}
