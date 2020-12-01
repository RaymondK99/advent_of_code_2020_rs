use std::io::prelude::*;
use std::env;

mod util;

use util::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("program <day> <part>");
        std::process::exit(1);
    }

    // Read arguments
    let day = args[1].parse::<u8>().unwrap();
    let part = match args[2].parse::<u8>() {
        Ok(1) => Part::Part1,
        Ok(2) => Part::Part2,
        _ => panic!("illegal part arguments!")
    };

    // Read input
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to fetch input...");

    let result = get_solution(day, part, input);

    println!("{}",result);

}
