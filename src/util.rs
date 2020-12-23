mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
//mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

mod day_20_2;



mod boot_code;

#[derive(PartialEq,Copy, Clone)]
pub enum Part {
    Part1,
    Part2,
}



pub fn get_solution(day:u8, part:Part, input:String) -> String {
    match day {
        1 => day_01::solve(input, part),
        2 => day_02::solve(input, part),
        3 => day_03::solve(input, part),
        4 => day_04::solve(input, part),
        5 => day_05::solve(input, part),
        6 => day_06::solve(input, part),
        7 => day_07::solve(input, part),
        8 => day_08::solve(input, part),
        9 => day_09::solve(input, part),
        10 => day_10::solve(input, part),
        11 => day_11::solve(input, part),
        12 => day_12::solve(input, part),
        //13 => day_13::solve(input, part),
        14 => day_14::solve(input, part),
        15 => day_15::solve(input, part),
        16 => day_16::solve(input, part),
        17 => day_17::solve(input, part),
        18 => day_18::solve(input, part),
        19 => day_19::solve(input, part),
        20 => day_20::solve(input, part),
        21 => day_21::solve(input, part),
        22 => day_22::solve(input, part),
        23 => day_23::solve(input, part),
        24 => day_24::solve(input, part),
        25 => day_25::solve(input, part),
        _ => panic!("..."),
    }

}




