use super::Part;
use util::day_19::Condition::{ExactMatch, MatchesRules, MatchesOr};
use std::collections::{HashMap};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    };

    result.to_string()
}

#[derive(Debug,Eq, PartialEq,Ord, PartialOrd)]
enum Condition {
    ExactMatch(char),
    MatchesRules(Vec<usize>),
    MatchesOr(Vec<usize>, Vec<usize>),
}

impl Condition {
    fn parse(input:&str) -> Condition {
        if input.contains("\"") {
            ExactMatch(input.trim().as_bytes()[1] as char)
        } else if input.contains("|") {
            let mut it = input.trim().split('|').into_iter();
            let cond1:Vec<usize> = it.next().unwrap().trim().split_ascii_whitespace().map(|s|s.parse().ok().unwrap()).collect();
            let cond2:Vec<usize> = it.next().unwrap().trim().split_ascii_whitespace().map(|s|s.parse().ok().unwrap()).collect();

            Condition::MatchesOr(cond1,cond2)

        } else {
            let conditions:Vec<usize> =  input.trim().split_ascii_whitespace().map(|s|s.parse().ok().unwrap()).collect();
            Condition::MatchesRules(conditions)
        }
    }
}

#[derive(Debug,Eq, PartialEq,Ord, PartialOrd)]
struct Rule {
    number:usize,
    condition:Condition,
}


struct Graph {
    rules:HashMap<usize, Rule>,
}

impl Graph {
    fn new(rules:HashMap<usize, Rule>) -> Graph {
        Graph{rules }
    }

    fn get_rule(&self,rule_no:usize) -> &Rule {
        self.rules.get(&rule_no).unwrap()
    }

    fn gen_permutations(query:&str,n:usize) -> Vec<Vec<&str>>{
        let mut permutations = vec![];

        if n == 1 {
            //query
            permutations.push(vec![query]);
        } else if n == 2 && query.len() >= 2 {
            for i in 1..query.len() {
                let mut list = vec![];
                list.push(&query[0..i]);
                list.push(&query[i..]);
                permutations.push(list);
            }

        }  else if n == 3 && query.len() >= 3 {
            for i in 1..query.len()-1 {
                for j in i+1..query.len() {
                    let mut list =  vec![];
                    list.push(&query[0..i]);
                    list.push(&query[i..j]);
                    list.push(&query[j..]);
                    permutations.push(list);
                }
            }
        }

        permutations
    }

    fn match_branch(&self,branches:&Vec<Vec<&str>>, rule_list:&Vec<usize>, cached_result:&mut HashMap<(usize,String),bool>) -> bool {
        branches.iter()
            .any( |branch| { (0..branch.len()).into_iter()
                .all( |index| self.search_internal(rule_list[index], branch[index], cached_result)) })
    }


    fn search(&self,rule_no:usize, searched_query:&str) -> bool {
        let mut cached_res = HashMap::new();
        self.search_internal(rule_no, searched_query, &mut cached_res)
    }


    fn search_internal(&self,rule_no:usize, searched_query:&str, cached_result:&mut HashMap<(usize,String),bool>) -> bool {

        let rule =  self.get_rule(rule_no);

        //println!("Query Rule {} matches:{}",rule_no, searched_query );
        if let Some(cached_res) = cached_result.get(&(rule_no, searched_query.to_string())) {
            //println!("===> [Cached] Rule {} matches:{} => {}",rule_no, searched_query, *cached_res );
            return *cached_res
        }

        let result = match &rule.condition {
            ExactMatch(ch) => {
                searched_query.len() == 1 && searched_query.as_bytes()[0] as char == *ch
            },
            MatchesRules(rule_list) => {
                let branches = Graph::gen_permutations(searched_query, rule_list.len());

                self.match_branch(&branches, rule_list, cached_result)
            },
            MatchesOr(rule_list1, rule_list2) => {
                let branches_left = Graph::gen_permutations(searched_query, rule_list1.len());
                let branches_right = Graph::gen_permutations(searched_query, rule_list2.len());

                self.match_branch(&branches_left, rule_list1, cached_result) || self.match_branch(&branches_right, rule_list2, cached_result)
            },
        };

        //println!("===> Rule {} matches:{} => {}",rule_no, searched_query, result );
        cached_result.insert((rule_no, searched_query.to_string()), result);
        result
    }
}


impl Rule {
    fn parse(input:&str) -> Rule {
        let mut it = input.split(':').into_iter();
        let number:usize = it.next().unwrap().parse().ok().unwrap();
        let condition = Condition::parse(it.next().unwrap());
        Rule{number,condition}
    }

}


fn parse(input:String) -> (HashMap<usize, Rule>, Vec<String>) {
    let mut it = input.split("\n\n").into_iter();
    let rules = it.next().unwrap();
    let data = it.next().unwrap();

    let mut rule_list:Vec<Rule> = rules.lines().map(|line| Rule::parse(line)).collect();
    let mut rule_map = HashMap::new();

    for rule in rule_list {
        rule_map.insert(rule.number, rule);
    }

    (rule_map, data.lines().map(|l|l.to_string()).collect())
}



fn part1(input:String) -> usize {
    let (rules,data) = parse(input.to_string());
    let mut graph = Graph::new(rules);

    data.iter().filter(|line| graph.search(0, line.as_str())).count()
}

fn part2(input:String) -> usize {
    let (rules,data) = parse(input.to_string());
    let mut graph = Graph::new(rules);

    let rule_8 = Rule{number:8, condition:Condition::MatchesOr(vec![42],vec![42,8])};
    let rule_11 = Rule{number:11, condition:Condition::MatchesOr(vec![42,31],vec![42,11,31])};

    graph.rules.insert(rule_8.number, rule_8);
    graph.rules.insert(rule_11.number, rule_11);

    data.iter().filter(|line| graph.search(0, line.as_str())).count()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test11() {

        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
6: 5
7: 4 5
8: 2 3
9: 4 1

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        let (rules,_) = parse(input.to_string());
        let mut graph = Graph::new(rules);

        assert_eq!(true,graph.search(0,"ababbb"));
        assert_eq!(true,graph.search(1,"aaab"));
        assert_eq!(true,graph.search(9,"ababb"));

        assert_eq!(true,graph.search(0,"ababbb"));
        assert_eq!(true,graph.search(0,"abbbab"));
        assert_eq!(false,graph.search(0,"bababa"));
        assert_eq!(false,graph.search(0,"aaabbb"));
        assert_eq!(false,graph.search(0,"aaaabbb"));
    }

    #[test]
    fn test13() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
6: 5
7: 4 5
8: 2 3

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        let (rules,_) = parse(input.to_string());
        let mut graph = Graph::new(rules);

        assert_eq!(true, graph.search(3, "ab"));
        assert_eq!(true, graph.search(3, "ba"));
        assert_eq!(false, graph.search(3, "a"));

        assert_eq!(true, graph.search(7, "ab"));
        assert_eq!(false, graph.search(7, "ba"));
        assert_eq!(false, graph.search(7, "a"));

        assert_eq!(true, graph.search(2, "aa"));
        assert_eq!(true, graph.search(2, "bb"));
        assert_eq!(true, graph.search(6, "b"));
        assert_eq!(false, graph.search(6, "a"));
        assert_eq!(false, graph.search(6, "ab"));
        assert_eq!(false, graph.search(6, "aa"));

    }

    #[test]
    fn test45() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        let (rules,_) = parse(input.to_string());
        let mut graph = Graph::new(rules);

        assert_eq!(false, graph.search(5, "a"));
        assert_eq!(true, graph.search(4, "a"));

        assert_eq!(true, graph.search(3, "ab"));
        assert_eq!(true, graph.search(3, "ba"));
        assert_eq!(false, graph.search(3, "aa"));

    }


    #[test]
    fn test46() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        let res = part1(input.to_string());
        assert_eq!(2,res);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_19.txt");

        //assert_eq!(224,part1(input.to_string()));
    }

    #[test]
    fn test2() {

        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        assert_eq!(3, part1(input.to_string()));
    }

    #[test]
    fn test22() {

        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        assert_eq!(12, part2(input.to_string()));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_19.txt");

        //assert_eq!(2,part2(input.to_string()));
    }

}
