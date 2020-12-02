use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let list = input.lines()
        .map(|line| parse_line(line))
        .collect();

    let result = match part {
        Part::Part1 => part1(list),
        Part::Part2 => part2(list)
    };

    format!("{}",result)
}

fn parse_line(input:&str) -> (usize,usize,char,&str) {
    let cols:Vec<&str> = input.split(|c| c == ' ' || c == ':' || c == '-' ).collect();
    let min = cols[0].parse().unwrap();
    let max = cols[1].parse().unwrap();
    let pattern = cols[2].as_bytes()[0] as char;
    let password = cols[4];

    (min,max,pattern,password)
}


fn part1(list:Vec<(usize,usize,char,&str)>) -> usize {
    list.iter().filter( |&(min,max,ch,input)| {
        let cnt = input.chars().filter(|c| *c == *ch).count();
        cnt >= *min && cnt <= *max
    }).count()
}

fn part2(list:Vec<(usize,usize,char,&str)>) -> usize {
    list.iter().filter( |&(index1,index2,ch,input)| {
        let ch1 = input.as_bytes()[index1-1] as char;
        let ch2 = input.as_bytes()[index2-1] as char;
        (*ch == ch1) ^ (*ch == ch2)
    }).count()}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test_parse() {
        let input = "1-3 a: abcde";
        assert_eq!((1,3,'a',"abcde"), parse_line(input));
    }

    #[test]
    fn test1() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

        assert_eq!("2", solve(input.to_string(),Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_02.txt");

        assert_eq!("548", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

        assert_eq!("1", solve(input.to_string(),Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_02.txt");

        assert_eq!("502", solve(input.to_string(), Part2));
    }

}
