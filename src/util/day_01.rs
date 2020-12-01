use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let numbers:Vec<u32> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let result = match part {
        Part::Part1 => part1(numbers),
        Part::Part2 => part2(numbers)
    };

    format!("{}",result)
}

fn build_k(list:Vec<u32>) -> Vec<bool> {
    let mut k = vec![false; 2021];

    list.iter()
        .filter(|n| **n <= 2020)
        .for_each( |n | k[*n as usize] = true);

    k
}

fn find_sum(start_index:usize, sum:usize, k:&Vec<bool>) -> Option<(usize, usize)> {
    for i in start_index..=sum as usize {
        if k[i] && k[sum-i] {
            return Some((i, sum - i))
        }
    }

    None
}

fn part1(list:Vec<u32>) -> String {
    let k = build_k(list);
    let ( a, b) = find_sum(0,2020, &k).unwrap();

    (a as u32 * b as u32).to_string()
}

fn part2(list:Vec<u32>) -> String {
    let k = build_k(list);

    for i in 0..2020 {
        if k[i] {
            // Is there a sum i + a + b = 2020 ?
            let res = find_sum(i+1, 2020 - i, &k);
            if res.is_some() {
                let (a,b) = res.unwrap();
                //println!("Found {}, {}, {}", i, a, b);
                return (a as u64 * b as u64 * i as u64).to_string()
            }
        }
    }

    String::from("N.A")
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "1721
979
366
299
675
1456";

        assert_eq!("514579", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_01.txt");

        assert_eq!("787776", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "1721
979
366
299
675
1456";

        assert_eq!("241861950", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_01.txt");

        assert_eq!("262738554", solve(input.to_string(), Part2));
    }

}
