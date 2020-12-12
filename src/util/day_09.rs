use super::Part;
use std::collections::VecDeque;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(&input,25),
        Part::Part2 => part2(input, 25),
    };

    result.to_string()
}



fn has_sum(preamble:&VecDeque<u64>,number:u64) -> bool {
    for i in 0..preamble.len()-1 {
        for j in i+1..preamble.len() {
            if preamble[i] + preamble[j] == number {
                return true;
            }
        }
    }
    false
}

fn find_sum(numbers:Vec<u64>,sum:u64) -> Vec<u64> {

    // Iterate through start position
    for i in 0..numbers.len() {
        let mut acc_sum = 0;
        let mut j = i;

        // Acc sum
        while acc_sum < sum {
            acc_sum += numbers[j];
            j += 1;
        }

        // Found sum?
        if acc_sum == sum {
            return numbers[i..j].to_vec()
        }
    }

    vec![]
}

fn part1(input:&str, problem_size:usize) -> u64 {
    let mut input_numbers:VecDeque<u64> = input.lines().map(|line| line.parse().ok().unwrap()).collect();
    let mut preamable:VecDeque<u64> = VecDeque::new();

    while !input_numbers.is_empty() {
        // Prepare preamable
        while preamable.len() < problem_size && !input_numbers.is_empty() {
            preamable.push_back(input_numbers.pop_front().unwrap());
        }

        // Evaluate number
        if let Some(next) = input_numbers.front() {
            if !has_sum(&preamable, *next) {
                return *next;
            }
        }

        // Drop last number in preamable
        preamable.pop_front();
    }

    1
}

fn part2(input:String,problem_size:usize) -> u64 {
    let result_part1 = part1(&input, problem_size);
    let sequence = find_sum(input.lines().map(|line| line.parse().ok().unwrap()).collect(), result_part1);

    sequence.iter().min().unwrap() + sequence.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let res = part1(&input.to_string(),5);
        assert_eq!(127,res);

    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_09.txt");
        let res = part1(&input.to_string(),25);
        assert_eq!(90433990,res);
    }


    #[test]
    fn test2() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let res = part2(input.to_string(),5);
        assert_eq!(62,res);
    }


    #[test]
    fn test_part2() {
        let input = include_str!("../../input_09.txt");
        let res = part2(input.to_string(),25);
        assert_eq!(11691646,res);
    }

}
