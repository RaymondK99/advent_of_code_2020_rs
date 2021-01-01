use super::Part;


pub fn solve(input : String, part: Part) -> String {
    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}

fn calc_period(a:i64, a_m:i64, b:i64, b_m:i64) -> (i64,i64) {
    let mut n = 0;
    let mut k = 0;
    let mut m = 0;

    loop {
        let t = k * a + a_m;
        if (t + b_m)  % b == 0 {
            if n == 0 {
                m = t;
            } else if n == 1 {
                return (t-m,m)
            }
            n += 1;
        }

        k +=1;
    }
}


fn parse(input:String) -> (i64, Vec<String>) {
    let mut it = input.lines().into_iter();
    let timestamp =  it.next().unwrap().parse().ok().unwrap();
    let bus_ids = it.next().unwrap().split(',').map(|s|s.to_string()).collect();
    (timestamp, bus_ids)
}

fn part1(input:String) -> i64 {
    let (timestamp, bus_ids) = parse(input);
    let ids:Vec<i64> = bus_ids.into_iter()
        .filter(|s|!s.eq(&"x"))
        .map(|s|s.parse().ok().unwrap())
        .collect();

    let mut min_waiting_time = i64::max_value();
    let mut res = 0;
    for id in ids {
        let t = id + (timestamp / id) * id;
        let delta = t - timestamp;

        if delta < min_waiting_time {
            min_waiting_time = delta;
            res = id * delta;
        }
    }

    res
}


fn part2(input:String) -> i64 {
    let (_, bus_ids) = parse(input);

    let v:Vec<(i64,i64)> = bus_ids.into_iter()
        .enumerate()
        .filter(|(_,s)| !s.starts_with("x"))
        .map(|(index,s)| (s.parse().ok().unwrap(), index as i64))
        .collect();

    let (_, m) = v.into_iter().fold((1,0), |(k1,m1), (k2,m2)| calc_period(k1,m1, k2,m2));
    m
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "939
7,13,x,x,59,x,31,19";

        assert_eq!(295, part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_13.txt");
        let res = part1(input.to_string());
        assert_eq!(119,res);
    }

    #[test]
    fn test20() {
        let input = "1\n7,13,x,x,59,x,31,19";
        let res = part2(input.to_string());
        assert_eq!(1068781,res);
    }

    #[test]
    fn test21() {
        let input = "1\n67,7,59,61";
        let res = part2(input.to_string());
        assert_eq!(754018,res);
    }

    #[test]
    fn test22() {
        let input = "1\n1789,37,47,1889";
        let res = part2(input.to_string());
        assert_eq!(1202161486,res);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_13.txt");
        let res = part2(input.to_string());
        assert_eq!(1106724616194525,res);
    }

}
