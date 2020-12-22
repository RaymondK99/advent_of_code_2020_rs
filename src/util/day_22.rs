use super::Part;
use std::collections::{VecDeque, HashSet};
use util::Part::{Part2, Part1};
use std::hash::Hash;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    result.to_string()
}

#[derive(Hash,Debug,Clone,Eq, PartialEq)]
struct GameState {
    player_one:VecDeque<usize>,
    player_two:VecDeque<usize>,
}

impl GameState {
    fn parse(input:&str) -> GameState {
        let mut it = input.split("\n\n");

        let player_one:  VecDeque<usize> = it.next().unwrap().lines()
            .filter(|s| !s.starts_with("Player"))
            .map(|s| s.parse().ok().unwrap())
            .collect();

        let player_two:  VecDeque<usize> = it.next().unwrap().lines()
            .filter(|s| !s.starts_with("Player"))
            .map(|s| s.parse().ok().unwrap())
            .collect();

        GameState{player_one,player_two}
    }

    fn game_ends(&self) -> bool {
        self.player_one.is_empty() || self.player_two.is_empty()
    }

    fn score_player(&self, player:&VecDeque<usize>) -> usize {
        player.iter().rev().enumerate().map(|(index,card)| *card * (index+1)).sum()
    }

    fn score(&self) -> usize {
        match !self.player_one.is_empty() {
            true => self.score_player(&self.player_one),
            false => self.score_player(&self.player_two),
        }
    }
}


fn play_game(mut game_state:GameState, mut _round:usize, part:Part) -> (bool,usize) {
    let mut player_one_cache:HashSet<VecDeque<usize>> = HashSet::new();
    let mut player_two_cache:HashSet<VecDeque<usize>> = HashSet::new();


    while !game_state.game_ends() {

        // Is this configuration already played?
        if part == Part2 && (player_one_cache.contains(&game_state.player_one) || player_two_cache.contains(&game_state.player_two)) {
            // Player one wins
            return (true, game_state.score_player(&game_state.player_one))
        }

        let game_state_before = game_state.clone();

        let player_one_card = game_state.player_one.pop_front().unwrap();
        let player_two_card = game_state.player_two.pop_front().unwrap();

        // Recursive game
        let player_one_wins = if part == Part2 && player_one_card <= game_state.player_one.len() && player_two_card <= game_state.player_two.len() {
            let mut game_state_rec = game_state.clone();
            game_state_rec.player_one.resize(player_one_card, 0);
            game_state_rec.player_two.resize(player_two_card, 0);

            let (player_one_wins, _) = play_game(game_state_rec, 1, Part2);
            player_one_wins
        } else {
            // Normal game
            player_one_card > player_two_card
        };

        if player_one_wins {
            game_state.player_one.push_back(player_one_card);
            game_state.player_one.push_back(player_two_card);
        } else {
            game_state.player_two.push_back(player_two_card);
            game_state.player_two.push_back(player_one_card);
        }

        player_one_cache.insert(game_state_before.player_one);
        player_two_cache.insert(game_state_before.player_two);
        _round += 1;
    }

    // Return (player one wins?, score of winner)
    (game_state.player_two.is_empty(), game_state.score())
}

fn part1(input:&str) -> usize {
    play_game(GameState::parse(input),1, Part1).1
}

fn part2(input:&str) -> usize {
    play_game(GameState::parse(input),1, Part2).1
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

        assert_eq!(105,part2(input));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_22.txt");

        assert_eq!(35836,part2(input));
    }



}
