use super::Part;
use std::collections::{VecDeque, HashMap};

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
    fn parse_line(line:&str) -> Rule {
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

    while let Some(current_rule) =  queue.pop_front() {

        if current_rule.bag.eq("shiny gold") {
            continue;
        }

        //println!("Eval {}",current_rule.bag);

        // Is current rule visited?
        if has_gold_map.get(&current_rule.bag).is_some() {
            // Already evaluated
            continue;
        } else {
            // Evaluate children
            if current_rule.content.is_empty() {
                // Does not contain gold
                has_gold_map.insert(&current_rule.bag, false);
                //println!("---> {} empty -> does NOT have gold",current_rule.bag);
            } else {

                // Child has gold?
                let child_has_gold = current_rule.content.iter().filter(|&(_,sub_bag)| sub_bag.eq("shiny gold")).count() > 0;
                if child_has_gold {
                    //println!("---> {} HAS gold directly",current_rule.bag);
                    has_gold_map.insert(&current_rule.bag, true);
                    continue;
                }

                // Child has gold indirectly
                let child_has_gold_indirectly = current_rule.content.iter()
                    .filter(|&(_,sub_bag)| has_gold_map.get(sub_bag).is_some() && *has_gold_map.get(sub_bag).unwrap()).count() > 0;

                if child_has_gold_indirectly {
                    //println!("---> {} HAS gold Indirectly",current_rule.bag);
                    has_gold_map.insert(&current_rule.bag, true);
                    continue;
                }

                // Do we need to evaluate the children rules?
                let re_eval:Vec<&Rule> = current_rule.content.iter()
                    .filter(|&(_,sub_bag)| has_gold_map.get( sub_bag).is_none())
                    .map( |(_,sub_bag)| *rule_book.get(sub_bag).unwrap())
                    .collect();

                re_eval.iter().for_each(|sub_rule| queue.push_front(sub_rule));

                if !re_eval.is_empty() {
                    // Evaluate rule later, after children nodes are done
                    queue.push_back(current_rule);
                    //println!("---> {} Re-eval",current_rule.bag);
                } else {
                    // No child elements has gold
                    has_gold_map.insert(&current_rule.bag, false);
                }
            }
        }
    }

    has_gold_map.iter().filter(|&(_,b)| *b).count()
}


fn count_bags(bag_name:&str,rule_book:&HashMap<String, &Rule>) -> usize {
    let mut cnt = 1;
    let rule = *rule_book.get(bag_name).unwrap();
    rule.content.iter().for_each(|(qty,sub_bag)| {
        cnt += *qty * count_bags(sub_bag, rule_book);
    });

    cnt
}

fn part1(input:String) -> usize {
    let rules:Vec<Rule> = input.lines().map(|line| Rule::parse_line(line)).collect();
    search_gold(rules)
}

fn part2(input:String) -> usize {
    let rules:Vec<Rule> = input.lines().map(|line| Rule::parse_line(line)).collect();
    let mut rule_book:HashMap<String,&Rule> = HashMap::new();
    rules.iter().for_each(|rule| {rule_book.insert(rule.bag.clone(), rule);});

    count_bags( "shiny gold",&rule_book)-1
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

        let rules:Vec<Rule> = input.lines().map(|line| Rule::parse_line(line)).collect();

        assert_eq!(4,search_gold(rules));

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


        let rules:Vec<Rule> = input.lines().map(|line| Rule::parse_line(line)).collect();
        let mut rule_book:HashMap<String,&Rule> = HashMap::new();
        rules.iter().for_each(|rule| {rule_book.insert(rule.bag.clone(), rule);});

        assert_eq!(33,count_bags( "shiny gold",&rule_book));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_07.txt");

        assert_eq!(8015,part2(input.to_string()));

    }
}
