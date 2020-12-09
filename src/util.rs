mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod boot_code;

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
        _ => panic!("..."),
    }

}




