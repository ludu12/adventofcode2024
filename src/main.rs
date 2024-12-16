use std::env;

mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut num:  u32 = 3;
    if args.len() > 1 {
        num = args[1].parse::<u32>().expect("Input should be a number");
    }

    println!("Running Advent of Code 2023. Day {}\n", num.to_string());

    match num {
        1 => one::run(),
        2 => two::run(),
        3 => three::run(),
        4 => four::run(),
        5 => five::run(),
        6 => six::run(),
        _ => println!("Invalid advent number! \"{}\"", num),
    }
}

