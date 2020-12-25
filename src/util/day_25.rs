use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => panic!("Not supported for day 25!!!"),
    };

    result.to_string()
}

const DIVISOR:usize = 20201227;
const SUBJECT:usize = 7;

fn parse(input:String) -> (usize, usize) {
    let mut it = input.lines().map(|line| line.parse().ok().unwrap()).into_iter();
    (it.next().unwrap(), it.next().unwrap())
}


fn derive_loop_size(public_key:usize, subject:usize) -> usize {
    let mut loop_size = 0;
    let mut value = 1;

    while value != public_key {
        value = value * subject;
        value = value % DIVISOR;
        loop_size += 1;
    }

    loop_size
}

fn transform(loop_size:usize, subject:usize) -> usize {
    (0..loop_size).into_iter().fold(1,|acc, _next| acc * subject % DIVISOR)
}

fn part1(input:String) -> usize {
    let (card_pub_key, door_pub_key) = parse(input);
    let loop_size_card = derive_loop_size(card_pub_key, SUBJECT);

    transform(loop_size_card, door_pub_key)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let input = "5764801\n17807724";

        assert_eq!(14897079,part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_25.txt");

        assert_eq!(18862163,part1(input.to_string()));
    }

}
