use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}


fn convert_bitset(input:&str) -> usize {
    input.chars().rev()
        .map(|c| match c {
            'B'|'R' => 1,
            'L'|'F' => 0,
            _ => panic!("Illegal character")
        })
        .enumerate()
        .map(|(bit_no,set)| set << bit_no)
        .sum()
}

fn convert_to_seat_id(input:&str) -> usize {
    convert_bitset(&input[0..7]) * 8 + convert_bitset(&input[7..10])
}


fn part1(input:String) -> usize {
    input.lines()
        .map(|line| convert_to_seat_id(line))
        .max().unwrap()
}

fn part2(input:String) -> usize {
    let mut seats:Vec<usize> = input.lines()
        .map(|line| convert_to_seat_id(line))
        .collect();

    // Sort it...
    seats.sort_unstable();

    *seats.iter()
        .enumerate()
        .find( |&(index, curr)| *curr != seats[index+1] - 1)
        .unwrap().1 + 1
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        assert_eq!(357, convert_to_seat_id("FBFBBFFRLR"));
        assert_eq!(567, convert_to_seat_id("BFFFBBFRRR"));
        assert_eq!(119, convert_to_seat_id("FFFBBBFRRR"));
        assert_eq!(820, convert_to_seat_id("BBFFBBFRLL"));

    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_05.txt");

        assert_eq!("974", solve(input.to_string(), Part1));

    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_05.txt");

        assert_eq!("646", solve(input.to_string(), Part2));

    }

}
