use super::Part;
use std::collections::{VecDeque, HashSet, HashMap};
use util::Part::{Part2, Part1};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    result.to_string()
}

fn parse(input:&str) -> (VecDeque<usize>,VecDeque<usize>) {
    let mut it = input.split("\n\n");

    let player_one:  VecDeque<usize> = it.next().unwrap().lines()
        .filter(|s| !s.starts_with("Player"))
        .map(|s| s.parse().ok().unwrap())
        .collect();

    let player_two:  VecDeque<usize> = it.next().unwrap().lines()
        .filter(|s| !s.starts_with("Player"))
        .map(|s| s.parse().ok().unwrap())
        .collect();

    (player_one, player_two)
}


fn play_game(mut player_one:VecDeque<usize>, mut player_two:VecDeque<usize>, mut round:usize, part:Part) -> (bool,usize) {
    let mut configutations = HashSet::new();


    while !player_one.is_empty() && !player_two.is_empty() {
        // Is this configuration already played
        //println!("------------------------\nPlayer one:{:?}, player two:{:?}", player_one, player_two);

        let player_one_before = player_one.clone();
        let player_two_before = player_two.clone();

        let player_one_card = player_one.pop_front().unwrap();
        let player_two_card = player_two.pop_front().unwrap();

        let player_one_wins = if part == Part2 && configutations.contains(&(player_one_before.clone(), player_two_before.clone()))  {
            //println!("===> Prev Game, player one wins:{}", true);
            // Player one wins
            true
        } else {

            // Recursive game
            if part == Part2 && player_one_card <= player_one.len() && player_two_card <= player_two.len() {
                //println!("====> Start Recursive game ");
                let mut player_one_rec = player_one.clone();
                let mut player_two_rec = player_two.clone();
                player_one_rec.resize(player_one_card,0);
                player_two_rec.resize(player_two_card, 0);

                let (player_one_wins, _) = play_game(player_one_rec, player_two_rec, 1, Part2);
                //println!("====> Recursive game player one wins:{}", player_one_wins);

                player_one_wins
            } else {
                // Normal game
                player_one_card > player_two_card
                //println!("====> Normal game player one wins:{}", player_one_wins);

            }
        };

        if player_one_wins {
            player_one.push_back(player_one_card);
            player_one.push_back(player_two_card);
        } else {
            player_two.push_back(player_two_card);
            player_two.push_back(player_one_card);
        }

        configutations.insert((player_one_before, player_two_before));
        round += 1;
    }

    // Return (player one wins?, score of winner)
    if player_two.is_empty() {
        (true, player_one.iter().rev().enumerate().map(|(index,card)| *card * (index+1)).sum())
    } else {
        (false,player_two.iter().rev().enumerate().map(|(index,card)| *card * (index+1)).sum())
    }
}

fn part1(input:&str) -> usize {
    let (player_one,player_two) = parse(input);
    play_game(player_one, player_two,1, Part1).1
}

fn part2(input:&str) -> usize {
    let (player_one,player_two) = parse(input);
    //println!("{:?},{:?}", player_one, player_two);
    play_game(player_one, player_two, 1, Part2).1
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

        assert_eq!(306,part1(input));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_22.txt");

        assert_eq!(32598,part1(input));
    }


    #[test]
    fn test2() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

        assert_eq!(291,part2(input));
    }

    #[test]
    fn test21() {
        let input = "Player 1:
43
19

Player 2:
2
29
14";

        assert_eq!(273,part2(input));
    }

    //#[test]
    fn test_part2() {
        let input = include_str!("../../input_22.txt");

        assert_eq!(1,part2(input));
    }



}
