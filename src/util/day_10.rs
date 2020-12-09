use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}


fn part1(input:String) -> u64 {
    1
}

fn part2(input:String) -> u64 {
    2
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "";

        let res = part1(input.to_string());
        //assert_eq!(1,res);

    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_10.txt");
        let res = part1(input.to_string());
        //assert_eq!(1,res);
    }

    #[test]
    fn test2() {
        let input = "";

        let res = part2(input.to_string());
        //assert_eq!(1,res);

    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_10.txt");
        let res = part2(input.to_string());
        //assert_eq!(1,res);
    }


}
