use super::Part;
use std::collections::HashMap;

pub fn solve(input : String, part: Part) -> String {
    let map = parse(input);

    let result = match part {
        Part::Part1 => part1(map),
        Part::Part2 => part2(map)
    };

    format!("{}",result)
}


fn parse(input:String) -> HashMap<(usize,usize),char> {
    let mut map = HashMap::new();

    input.lines().enumerate()
        .flat_map(move |(y,line)| line.chars().enumerate().map(move |(x,ch)| (x,y,ch)))
        .for_each(|(x,y,ch) | {map.insert((x,y),ch);});

    map
}


fn calc_trees(incr_x:usize, incr_y:usize, map:&HashMap<(usize,usize),char>) -> u64 {
    let width = map.iter().map(|(&(x,_),_)|x).max().unwrap()+1;
    let height = map.iter().map(|(&(_,y),_)|y).max().unwrap()+1;

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < height {
        let pos = map.get(&(x % width,y)).unwrap();
        if *pos == '#' {
            count += 1;
            //println!("{},{} is tree",x+1,y+1);
        }

        x += incr_x;
        y += incr_y;
    }

    count
}

fn part1(map:HashMap<(usize,usize),char>) -> u64 {
    calc_trees(3,1,&map)
}

fn part2(map:HashMap<(usize,usize),char>) -> u64 {
    let incr_list = [(1,1),(3,1),(5,1),(7,1),(1,2)];
    incr_list.iter()
        .map( |(incr_x,incr_y)| calc_trees(*incr_x,*incr_y,&map))
        .fold(1,|acc, item| item * acc)

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        assert_eq!("7", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {

        let input = include_str!("../../input_03.txt");

        assert_eq!("252", solve(input.to_string(), Part1));
    }


    #[test]
    fn test2() {

        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        assert_eq!("336", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {

        let input = include_str!("../../input_03.txt");

        assert_eq!("2608962048", solve(input.to_string(), Part2));
    }

}
