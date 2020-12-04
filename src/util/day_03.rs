use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let map_tuple = parse(input);

    let result = match part {
        Part::Part1 => part1(map_tuple),
        Part::Part2 => part2(map_tuple)
    };

    format!("{}",result)
}


fn parse(input:String) -> (usize, usize, Vec<char>) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let map = input.lines().flat_map(move |line| line.chars()).collect();
    (width, height, map)
}


fn calc_trees(width:usize, height:usize, incr_x:usize, incr_y:usize, map:&[char]) -> u64 {

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < height {
        let pos = map.get( y*width + x % width).unwrap();
        if *pos == '#' {
            count += 1;
            //println!("{},{} is tree",x+1,y+1);
        }

        x += incr_x;
        y += incr_y;
    }

    count
}

fn part1(map_tuple:(usize, usize, Vec<char>)) -> u64 {
    calc_trees(map_tuple.0, map_tuple.1, 3,1,&map_tuple.2)
}

fn part2(map_tuple:(usize, usize, Vec<char>)) -> u64 {
    let incr_list = [(1,1),(3,1),(5,1),(7,1),(1,2)];
    incr_list.iter()
        .map( |(incr_x,incr_y)| calc_trees(map_tuple.0, map_tuple.1, *incr_x,*incr_y,&map_tuple.2))
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
