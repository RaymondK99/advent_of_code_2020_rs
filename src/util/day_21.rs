use super::Part;
use std::collections::{HashSet, HashMap};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    };

    result.to_string()
}

fn parse_line(input:&str) -> (Vec<&str>, Vec<&str>) {
    let mut it = input.split(|c| c == '(' || c == ')');
    let ingredients:Vec<&str> = it.next().unwrap().trim().split_ascii_whitespace().collect();
    let allergens:Vec<&str> = it.next().unwrap().trim().split(|c| c == ' ' || c == ',')
        .filter(|s|!s.starts_with("contains"))
        .filter(|s| s.len() > 0)
        .collect();

    (ingredients, allergens)
}


fn categorise(input:String) -> (Vec<String>, Vec<String>) {
    let list:Vec<(Vec<&str>,Vec<&str>)> = input.lines().map(|line| parse_line(line)).collect();

    // ex: dairy -> [aaa,bbb,ccc]
    let mut possible_matches:HashMap<String,Vec<&str>> = HashMap::new();

    for (ingredients,allergens) in list.iter() {
        // For each allergen
        for allergen in allergens.iter() {
            if let Some(allergen_matches) = possible_matches.get_mut(&allergen.to_string()) {
                // Update item by using the intersection of the 2 lists
                allergen_matches.retain( |item| ingredients.contains(item));
            } else {
                possible_matches.insert(allergen.to_string(), ingredients.clone());
            }
        }
    }

    // Reduce allergen map
    'outer: loop {
        let exact_matches: Vec<(String, Vec<&str>)> = possible_matches.iter()
            .filter(|item| item.1.len() == 1)
            .map(|(s, v)| (s.clone(), v.clone()))
            .collect();

        for (allergen, list) in exact_matches {
            let ingredient = list.first().unwrap();
            for (key, value) in possible_matches.iter_mut() {
                if !allergen.eq(key) && value.iter().find(|&item| item.eq(ingredient)).is_some() {
                    value.retain(|item| !ingredient.eq(item));
                    continue 'outer;
                }
            }
        }

        break;
    }

    // Produce list allergens
    let mut dangerous_list_key:Vec<String> = possible_matches.keys().map(|k| k.to_string()).collect();
    dangerous_list_key.sort_unstable();

    // Produce list of items that contains allergens
    let danger_list:Vec<String>  = dangerous_list_key.iter()
        .map( |key| possible_matches.get(key).unwrap().first().unwrap().to_string())
        .collect();


    let matched:HashSet<String> = possible_matches.values().flat_map(|v| v.iter().map(|s| s.to_string())).collect();

    let unmatched:Vec<String> = list.iter()
        .map(|(i,_)| i)
        .flat_map(|i| i.iter().map(|s|s.to_string()))
        .filter(| item| !matched.contains(item))
        .collect();


    (unmatched, danger_list)
}

fn part1(input:String) -> String {
    let (unmatched,_) = categorise(input);
    unmatched.len().to_string()
}

fn part2(input:String) -> String {
    let (_,danger_list) = categorise(input);
    danger_list.iter().fold( String::new(), |acc,b | {
        if acc.is_empty() {
            b.to_string()
        } else {
            acc + "," + b
        }
    })
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        assert_eq!("5",part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_21.txt");

        assert_eq!("1945",part1(input.to_string()));
    }


    #[test]
    fn test2() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        assert_eq!("mxmxvkd,sqjhc,fvjkl",part2(input.to_string()));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_21.txt");

        assert_eq!("pgnpx,srmsh,ksdgk,dskjpq,nvbrx,khqsk,zbkbgp,xzb",part2(input.to_string()));
    }

}
