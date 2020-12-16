use super::Part;
use std::collections::{HashSet, VecDeque};


pub fn solve(input : String, part: Part) -> String {
    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input,"departure"),
    };

    result.to_string()
}

#[derive(Debug)]
struct Rule {
    name:String,
    interval1:(u64,u64),
    interval2:(u64,u64),
}

impl Rule {
    fn parse(line:&str) -> Rule {
        let cols:Vec<&str> = line.split(':').collect();
        let name = String::from(cols[0]);
        let next:Vec<&str> = cols[1].split(|c| c == ' ' || c == '-').collect();

        let interval1 = (next[1].parse().ok().unwrap(), next[2].parse().ok().unwrap());
        let interval2 = (next[4].parse().ok().unwrap(), next[5].parse().ok().unwrap());

        Rule{name,interval1,interval2 }
    }

    fn field_valid(&self, num:u64) -> bool {
        (num >= self.interval1.0 && num <= self.interval1.1 ) || (num >= self.interval2.0 && num <= self.interval2.1)
    }

}

fn parse(input:String) -> (Vec<Rule>, Vec<u64>, Vec<Vec<u64>> ) {
    let sections:Vec<&str>  = input
        .split("\n\n")
        .collect();
    let rules:Vec<Rule> = sections[0].lines().map(|line|Rule::parse(line)).collect();

    let tickets : Vec<Vec<u64>> = sections[2].lines()
        .filter(|line| !line.starts_with("nearby"))
        .map(|line| line.split(',')
            .map(|item|item.parse().ok().unwrap())
            .collect())
        .collect();

    let your_ticket:Vec<u64> = sections[1].lines()
        .filter(|l|!l.starts_with("your")).into_iter().next().unwrap()
        .split(",").map(|s|s.parse().unwrap()).collect();

    (rules, your_ticket, tickets)
}

fn part1(input:String) -> u64 {
    let (rules,_,tickets) = parse(input);

    let mut sum = 0;

    tickets.iter().for_each(|ticket|{
        ticket.iter().for_each(|num| {
            if !rules.iter().map(|rule|rule.field_valid(*num)).fold(false,|acc,next| acc || next) {
                sum += num;
            }
        })
    });
    sum

}

fn is_ticket_valid(ticket:&Vec<u64>,rules:&Vec<Rule>) -> bool {
    ticket.iter()
        .map(|num| rules.iter().map(|rule| rule.field_valid(*num)).fold(false,|acc,next| acc || next) )
        .fold(true, |acc, next| acc && next)

}

fn get_rule_index_list(ticket:&Vec<u64>, rule:&Rule) -> HashSet<usize> {
    ticket.iter()
        .enumerate()
        .filter(|&(_,num)| rule.field_valid(*num))
        .map(|(index,_)| index )
        .collect()
}

fn get_rule_index(tickets:&Vec<&Vec<u64>>, rule:&Rule) -> HashSet<usize> {
    let mut set = HashSet::new();

    for &ticket in tickets.iter() {
        if set.is_empty() {
            set = get_rule_index_list(ticket,rule);
        } else {
            let next_set = get_rule_index_list(ticket,rule);
            set = set.intersection(&next_set).copied().collect();
        }
    }

    set
}

fn part2(input:String, filter_expr:&str) -> u64 {
    let (rules,your_ticket,tickets) = parse(input);

    let valid_tickets:Vec<&Vec<u64>> = tickets.iter()
        .filter(|&ticket| is_ticket_valid(ticket,&rules))
        .collect();


    let mut rule_candidates:VecDeque<(HashSet<usize>, &Rule)> = rules.iter()
        .map(|rule| (get_rule_index(&valid_tickets,rule),rule))
        .collect();


    let mut matched_rules = vec![];

    while let Some((set,rule)) = rule_candidates.pop_front() {

        // Distinct matched rule
        if set.len() == 1 {
            let col_number = set.iter().next().unwrap();
            rule_candidates.iter_mut().for_each(|(s,_)| {s.remove(col_number);});
            matched_rules.push((*col_number, rule));
        } else {
            rule_candidates.push_back((set, rule));
        }
    }

    matched_rules.iter()
        .filter(|(_,r)|r.name.starts_with(filter_expr))
        .map(|(index,_)| your_ticket[*index]).fold(1, |acc, next| acc * next)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test11() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        assert_eq!(71, part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_16.txt");
        let res = part1(input.to_string());
        assert_eq!(23036,res);
    }

    #[test]
    fn test2() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        assert_eq!(1716, part2(input.to_string(),""));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_16.txt");
        let res = part2(input.to_string(),"departure");
        assert_eq!(1909224687553,res);
    }

}
