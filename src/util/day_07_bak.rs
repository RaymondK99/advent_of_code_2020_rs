use super::Part;
use std::collections::{HashSet, VecDeque, HashMap};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}

#[derive(Debug,Hash,Eq, PartialEq)]
struct Rule {
    bag:String,
    content:Vec<(usize,String)>,
}

impl Rule {
    fn parseLine(line:&str) -> Rule {
        let mut it = line.split("contain");
        let mut bag_it = it.next().unwrap().split_ascii_whitespace();
        let bag_str = format!("{} {}",bag_it.next().unwrap(),bag_it.next().unwrap());
        let contents = it.next().unwrap().split(',');
        let mut children = vec![];
        for content in contents {
            if content.contains("no other") {
                continue;
            }
            let mut it = content.split_ascii_whitespace();
            let qty = it.next().unwrap().parse().ok().unwrap();
            let desc = it.next().unwrap();
            let color = it.next().unwrap();
            let child_bag_str = format!("{} {}",desc,color);
            children.push((qty, child_bag_str));
        };

        Rule{bag:bag_str, content:children}
    }
}

fn search_gold(rules:Vec<Rule>) -> usize {
    let mut rule_book:HashMap<String,&Rule> = HashMap::new();
    let mut has_gold_map:HashMap<&String,bool> = HashMap::new();
    let mut queue = VecDeque::new();

    rules.iter().for_each(|rule| queue.push_back(rule));
    rules.iter().for_each(|rule| {rule_book.insert(rule.bag.clone(), rule);});

    while !queue.is_empty() {
        let current_rule = queue.pop_front().unwrap();

        if current_rule.bag.eq("shiny gold") {
            continue;
        }

        println!("Eval {}",current_rule.bag);

        // Is current rule visited?
        if let Some(has_gold) = has_gold_map.get(&current_rule.bag) {
            // Already evaluated
            continue;
        } else {
            // Add children
            if current_rule.content.is_empty() {
                // Does not contain gold
                has_gold_map.insert(&current_rule.bag, false);
                println!("---> {} empty -> does NOT have gold",current_rule.bag);
            } else {

                // Has gold directly?
                let has_gold_directly = current_rule.content.iter().filter(|&(_,sub_bag)| sub_bag.eq("shiny gold")).count() > 0;
                if has_gold_directly {
                    println!("---> {} HAS gold directly",current_rule.bag);
                    has_gold_map.insert(&current_rule.bag, true);
                    continue;
                }

                let child_has_gold = current_rule.content.iter()
                    .filter(|&(_,sub_bag)| has_gold_map.get(sub_bag).is_some() && *has_gold_map.get(sub_bag).unwrap()).count()>0;

                if child_has_gold {
                    println!("---> {} HAS gold Indirectly",current_rule.bag);
                    has_gold_map.insert(&current_rule.bag, true);
                    continue;
                }

                let re_eval:Vec<&Rule> = current_rule.content.iter()
                    .filter(|&(_,sub_bag)| has_gold_map.get( sub_bag).is_none())
                    .map( |(_,sub_bag)| *rule_book.get(sub_bag).unwrap())
                    .collect();

                re_eval.iter().for_each(|sub_rule| queue.push_front(sub_rule));


                if !re_eval.is_empty() {
                    // Evaluate later
                    queue.push_back(current_rule);
                    println!("---> {} Re-eval",current_rule.bag);
                } else {
                    has_gold_map.insert(&current_rule.bag, false);
                }
            }
        }
    }

    has_gold_map.iter().filter(|&(_,b)| *b).count()
}



fn contains(searched_bag:&str, queried_bag:&str, rules_str:&str) -> bool {
    let rules:Vec<Vec<&str>> = rules_str.lines()
        .filter(|line| !line.starts_with(searched_bag))
        .filter(|line| line.starts_with(queried_bag))
        .filter(|line| !line.contains("no other bags."))
        .map( |line| line.split("contain").collect())
        .collect();

    for rule in rules {
        let contents = rule[1].split(',');
        for bag_str in contents {
            let mut it = bag_str.split_ascii_whitespace();
            let _qty = it.next().unwrap();
            let desc = it.next().unwrap();
            let color = it.next().unwrap();

            let next_query = format!("{} {}",desc,color);
            if next_query.eq(searched_bag) || contains(searched_bag, next_query.as_str(), rules_str) {
                return true;
            }
        }
    }

    false
}

fn bag_count( queried_bag:&str, rules_str:&str) -> usize {
    let mut cnt = 1;
    let rules:Vec<Vec<&str>> = rules_str.lines()
        .filter(|line| line.starts_with(queried_bag))
        .filter(|line| !line.contains("no other bags."))
        .map( |line| line.split("contain").collect())
        .collect();

    for rule in rules {
        let contents = rule[1].split(',');
        for bag_str in contents {
            let mut it = bag_str.split_ascii_whitespace();
            let qty_str = it.next().unwrap();
            let qty:usize = qty_str.parse().ok().unwrap();
            let desc = it.next().unwrap();
            let color = it.next().unwrap();

            let next_query = format!("{} {}",desc,color);
            let query_cnt = qty * bag_count( next_query.as_str(), rules_str);
            cnt += query_cnt;
        }
    }

    cnt
}

fn part1(input:String) -> usize {
    let rules:Vec<Rule> = input.lines().map(|line| Rule::parseLine(line)).collect();
    search_gold(rules)
}
fn part12(input:String) -> usize {
    let searched_bag = "shiny gold";
    input.lines()
        .filter(|line| !line.starts_with(searched_bag))
        .filter(|line| {
            let mut it = line.split_ascii_whitespace();
            let desc = it.next().unwrap();
            let color = it.next().unwrap();
            let query = format!("{} {}",desc,color);
            //println!(" ------> {} ",query);
            let res = contains(searched_bag, query.as_str(), input.as_str());
            //println!("{} = {}", query, res);
            res
        })
        .count()
}

fn part2(input:String) -> usize {
    let searched_bag = "shiny gold";
    bag_count(searched_bag, input.as_str()) - 1
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test11() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let rules:Vec<Rule> = input.lines().map(|line| Rule::parseLine(line)).collect();
        println!("{:?}",rules);

        search_gold(rules);

    }

    #[test]
    fn test12() {
        let input = include_str!("../../input_07.txt");

        let rules:Vec<Rule> = input.lines().map(|line| Rule::parseLine(line)).collect();
        println!("{:?}",rules);

        search_gold(rules);

    }
   // #[test]
    fn test1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";


        assert_eq!(true,contains("shiny gold","light red",input));
        assert_eq!(true,contains("shiny gold","muted yellow",input));
        assert_eq!(true,contains("shiny gold","dark orange",input));
        assert_eq!(true,contains("shiny gold","bright white",input));
        assert_eq!(false,contains("shiny gold","dark olive",input));
        assert_eq!(false,contains("shiny gold","vibrant plum",input));
        assert_eq!(false,contains("shiny gold","faded blue",input));
        assert_eq!(false,contains("shiny gold","dotted black",input));

        assert_eq!(4,part1(input.to_string()));

    }


    #[test]
    fn test_part11() {
        let input = include_str!("../../input_07.txt");

        assert_eq!(372,part1(input.to_string()));

    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_07.txt");

        assert_eq!(372,part1(input.to_string()));

    }

    #[test]
    fn test2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";


        assert_eq!(33,bag_count( "shiny gold",input));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_07.txt");

        assert_eq!(8015,part2(input.to_string()));

    }


}
