use super::Part;
use std::collections::VecDeque;


pub fn solve(input : String, part: Part) -> String {
    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}


fn insert_number(number:i64, turn:i64, history:&mut Vec<[i64;2]>) {
    if let Some(vec) = history.get_mut(number as usize) {
        // Shift out oldest turn, insert new
        vec[1] = vec[0];
        vec[0] = turn;
    } else {
        panic!("...");
    }
}

fn get_delta(number:i64, history:&[[i64;2]]) -> Option<i64> {
    if let Some(vec) = history.get(number as usize) {
        if vec[0] > 0 && vec[1] > 0 {
            return Some(vec[0] - vec[1])
        }
    }

    None
}

fn play_game(input:String, final_turn:i64) -> i64 {
    let mut start_numbers :VecDeque<i64>= input.split(',')
        .map(|s|s.trim().parse().ok().unwrap())
        .collect();

    let mut history: Vec<[i64;2]> = Vec::with_capacity(final_turn as usize);
    let mut last_number = 0;

    for _ in 0..=final_turn {
        history.push([0,0]);
    }

    for turn in 1..=final_turn {
        if let Some(number) = start_numbers.pop_front() {
            // Init initial numbers
            insert_number(number, turn, &mut history);
            last_number = number;
        } else {
            // Perform remaining turns
            if let Some(delta) = get_delta(last_number, &history) {
                last_number = delta;
            } else {
                last_number = 0;
            }

            // Update last number
            insert_number(last_number, turn, &mut history);
        }
    }


    last_number
}

fn part1(input:String) -> i64 {
    play_game(input, 2020)
}

fn part2(input:String) -> i64 {
    play_game(input, 30_000_000)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test1() {
        let input = "0,3,6";

        assert_eq!(436, part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_15.txt");
        let res = part1(input.to_string());
        assert_eq!(620,res);
    }

    //#[test]
    fn _test2() {
        let input = "0,3,6";

        assert_eq!(175594, part2(input.to_string()));
    }

    //#[test]
    fn _test_part2() {
        let input = include_str!("../../input_15.txt");
        let res = part2(input.to_string());
        assert_eq!(110871,res);
    }

}
