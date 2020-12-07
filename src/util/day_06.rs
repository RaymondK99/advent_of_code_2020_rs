use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}


fn count_yes(input:String, acc:u32, acc_func:fn(u32,u32) -> u32) -> u32 {
    input.split("\n\n")
        .map(|group_answer| group_answer.lines()
            .map(|line| line.chars().map(|ch| (1 << (ch as usize - 'a' as usize))).sum::<u32>())
            .fold(acc, |acc, a| acc_func(acc,a)).count_ones())
        .sum()
}

fn part1(input:String) -> u32 {
    count_yes(input, 0,|a,b| a | b)
}

fn part2(input:String) -> u32 {
    count_yes(input, u32::max_value(), |a,b| a & b)
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";


        assert_eq!(11, part1(input.to_string()));
        assert_eq!(6, part2(input.to_string()));

    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_06.txt");
        assert_eq!("6585", solve(input.to_string(), Part1));

    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_06.txt");
        assert_eq!("3276", solve(input.to_string(), Part2));

    }

}
