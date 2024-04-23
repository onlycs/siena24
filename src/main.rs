#![feature(let_chains)]

mod common;
mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;

use clap::Parser;

#[derive(Parser)]
#[command(version, about = "2024 Siena HSCS Competition Solutions")]
struct Args {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=7))]
    problem: u8,
}

fn main() {
    let args = Args::parse();

    match args.problem {
        1 => problem1::main(),
        2 => problem2::main(),
        3 => problem3::main(),
        4 => problem4::main(),
        5 => problem5::main(),
        6 => problem6::main(),
        7 => problem7::main(),
        _ => unreachable!(),
    }
}
