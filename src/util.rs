mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;


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
        _ => panic!("..."),
    }

}




