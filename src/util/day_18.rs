use super::Part;
use std::collections::VecDeque;
use util::Part::{Part1, Part2};
use std::cmp::max;


pub fn solve(input : String, part: Part) -> String {
    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}


fn parse(expr:String) -> VecDeque<String> {
    let mut out_terms = VecDeque::new();
    let mut i = 0;
    while i < expr.len() {

        let mut ch = expr.as_bytes()[i] as char;
        if ch.is_ascii_digit() {
            // Push digit
            let mut num_str = String::new();
            while i < expr.len() && ch.is_ascii_digit() {
                num_str.push(ch);
                i += 1;
                if i < expr.len() {
                    ch = expr.as_bytes()[i] as char;
                }
            }
            out_terms.push_back(num_str);
        } else if !ch.is_ascii_whitespace() {
            out_terms.push_back(String::from(ch));
            i += 1;
        } else {
            i += 1;
        }
    }

    out_terms
}

fn compute(mut terms:VecDeque<String>,part:Part) -> i64 {
    //println!("      ===> {:?}", terms);

    if part == Part1 {
        while reduce(&mut terms,false) {
            //println!("    ===> {:?}", terms);
        }

        terms.front().unwrap().parse().ok().unwrap()
    } else {

        let mut can_reduce = true;
        while can_reduce {
            while reduce(&mut terms, true) {
                //println!("  (+) ===> {:?}", terms);
            }

            can_reduce = reduce(&mut terms, false);
            if !can_reduce {
                //println!("      ===> {:?}", terms);
            }
        }

        terms.front().unwrap().parse().ok().unwrap()
    }
}

fn reduce(terms:&mut VecDeque<String>, only_plus:bool) -> bool {
    let mut tmp = VecDeque::new();
    let len = terms.len();

    let mut max_depth = 0;
    let mut curr_depth = 0;

    terms.iter()
        .for_each(|c| {
            if c.as_str().eq("(") {
                curr_depth += 1;
            } else if c.as_str().eq(")") {
                curr_depth -= 1;
            }

            max_depth = max(curr_depth, max_depth);
        });


    while terms.len() > 2 {
        let term1_is_digit = terms[0].chars().all(|c| c.is_ascii_digit());
        let term2_is_digit = term1_is_digit && terms[2].chars().all(|c| c.is_ascii_digit());

        if terms[0].eq("(") {
            curr_depth += 1;
        } else if terms[0].eq(")") {
            curr_depth -= 1;
        }

        if curr_depth < max_depth {
            tmp.push_back(terms.pop_front().unwrap());
            continue;
        }

        if terms[0].eq("(") && terms[1].eq(")") {
            // Remove '()'
            terms.pop_front();
            terms.pop_front();
            break;
        } else if terms[0].eq("(") && terms[2].eq(")") {
            // ( number) -> number
            terms.pop_front();
            let num_str = terms.pop_front().unwrap();
            terms.pop_front();
            terms.push_front(num_str);
            break;
        } else if term1_is_digit && term2_is_digit && (terms[1].eq("+") || (terms[1].eq("*") && !only_plus)) {
            // -> number +/* number -> number
            let lh:i64 = terms.pop_front().unwrap().parse().ok().unwrap();
            let op = terms.pop_front().unwrap();
            let rh:i64 = terms.pop_front().unwrap().parse().ok().unwrap();
            let tmp_res = match op.as_str() {
                "*" => lh * rh,
                "+" => lh + rh,
                _ => panic!(".."),
            };

            terms.push_front(tmp_res.to_string());
            break;
        } else {
            // Try next...
            tmp.push_back(terms.pop_front().unwrap());
        }
    }

    // Restore it...
    while tmp.is_empty() == false {
        terms.push_front(tmp.pop_back().unwrap());
    }

    // Did we reduce something?
    len != terms.len()
}



fn part1(input:String) -> i64 {
    input.lines().map(|line| compute(parse(line.to_string()), Part1)).sum()
}


fn part2(input:String) -> i64 {
    input.lines().map(|line| compute(parse(line.to_string()), Part2)).sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let input2 = "1 + (2 * 3) + (4 * (5 + 6))";

        assert_eq!(71, part1(input.to_string()));
        assert_eq!(51, part1(input2.to_string()));

    }

    #[test]
    fn test12() {
        let input2 = "(4 * (5 + 6))";
        assert_eq!(44, part1(input2.to_string()));
    }

    #[test]
    fn test13() {
        let input2 = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(51, part1(input2.to_string()));

    }

    #[test]
    fn test14() {
        let input2 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(13632, part1(input2.to_string()));

    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_18.txt");
        let res = part1(input.to_string());
        assert_eq!(131076645626,res);
    }

    #[test]
    fn test2() {
        let input = "2 * 3 + (4 * 5)";

        assert_eq!(46, part2(input.to_string()));
    }

    #[test]
    fn test21() {
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";

        assert_eq!(1445, part2(input.to_string()));
    }

    #[test]
    fn test22() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

        assert_eq!(23340, part2(input.to_string()));
    }


    #[test]
    fn test_part2() {
        let input = include_str!("../../input_18.txt");
        let res = part2(input.to_string());
        assert_eq!(109418509151782,res);
    }

}
