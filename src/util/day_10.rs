use super::Part;
use std::collections::HashMap;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}


fn part1(input:String) -> u64 {
    let mut adapters:Vec<u64> = input.lines().map(|line| line.parse().ok().unwrap()).collect();

    adapters.sort_unstable();

    let (ones,threes,_) = adapters.iter().fold((0,1,0) , |a,&b|{
        let (ones, threes, prev) = a;
        let diff = b - prev;
        match diff {
            1 => (ones+1,threes,b),
            3 => (ones,threes+1,b),
            _ => (ones,threes,b)
        }
    });

    ones * threes

}


fn connect(adapters:&Vec<u64>,cache:&mut HashMap<u64,u64>) -> u64 {

    // We reached the end
    if adapters.len() == 1 {
        return 1;
    } else if let Some(cached_res) = cache.get(&adapters[0]) {
        // Is there an already cached result for this sub-array?
        return *cached_res;
    }

    let mut index = 1;
    let mut count = 0;
    while index < adapters.len() &&  (adapters[index] - adapters[0]) < 4 {
        let res = connect(&adapters[index..].to_vec(),cache);
        // Cache result in order to reuse it later
        cache.insert(adapters[index],res);
        index += 1;
        count += res;
    }

    count
}

fn part2(input:String) -> u64 {
    let mut adapters:Vec<u64> = input.lines().map(|line| line.parse().ok().unwrap()).collect();
    let mut cache = HashMap::new();
    adapters.sort_unstable();
    
    let mut sum = 0;
    while *adapters.first().unwrap() < 4 {
        sum += connect(&adapters,&mut cache);
        adapters.remove(0);
    }

    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";

        let res = part1(input.to_string());
        assert_eq!(35,res);

    }

    #[test]
    fn test11() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        let res = part1(input.to_string());
        assert_eq!(220,res);

    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_10.txt");
        let res = part1(input.to_string());
        assert_eq!(2312,res);
    }

    #[test]
    fn test2() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        let res = part2(input.to_string());
        assert_eq!(8,res);

    }

    #[test]
    fn test21() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        let res = part2(input.to_string());
        assert_eq!(19208,res);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_10.txt");
        let res = part2(input.to_string());
        assert_eq!(12089663946752,res);
    }


}
