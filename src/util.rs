mod day_01;
mod day_02;
mod day_03;
mod day_04;


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
        _ => panic!("..."),
    }

}




