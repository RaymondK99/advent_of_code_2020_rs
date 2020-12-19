use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    };

    result.to_string()
}

fn part1(input:String) -> usize {
    1
}

fn part2(input:String) -> usize {
    1
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "";

        assert_eq!(1,part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_20.txt");

        assert_eq!(1,part1(input.to_string()));
    }


    #[test]
    fn test2() {
        let input = "";

        assert_eq!(1,part2(input.to_string()));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_20.txt");

        assert_eq!(1,part2(input.to_string()));
    }

}
