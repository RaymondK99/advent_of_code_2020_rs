use super::Part;
use std::collections::{HashMap};
use util::Part::{Part1, Part2};


pub fn solve(input : String, part: Part) -> String {
    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}

#[derive(Debug,Hash,Copy, Clone,Eq, PartialEq)]
struct Pos {
    x:i32,
    y:i32,
    z:i32,
    w:i32,
}

impl Pos {
    fn new(x:i32,y:i32) -> Pos {
        Pos{x,y, z: 0,w:0 }
    }


    fn get_neighbors_4d(&self) -> Vec<Pos> {
        let mut neighbors = vec![];
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dz == 0 && dx == 0 && dy == 0 && dw == 0 {
                            continue;
                        }

                        neighbors.push(Pos{x:self.x+dx,y:self.y+dy,z:self.z+dz,w:self.w+dw})
                    }
                }
            }
        }
        neighbors
    }

    fn get_neighbors_3d(&self) -> Vec<Pos> {
        let mut neighbors = vec![];
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                        if dz == 0 && dx == 0 && dy == 0 {
                            continue;
                        }
                        neighbors.push(Pos{x:self.x+dx,y:self.y+dy,z:self.z+dz,w:0})
                }
            }
        }
        neighbors
    }
}

fn parse(input:&str) -> HashMap<Pos,char> {
    let mut map = HashMap::new();

    input.lines()
        .enumerate()
        .flat_map(move|(y,line)| line.chars().enumerate()
            .map(move|(x,ch)| (Pos::new(x as i32,y as i32),ch)))
        .for_each(|(pos,ch)| {map.insert(pos,ch);});

    map
}


fn mutate(map:&mut HashMap<Pos,char>, part:Part) {
    let mut num_active_map = HashMap::new();

    // Initiate map of currently active neighbors with '0' for active cubes
    map.iter()
        .filter(|&(_,ch)| *ch == '#')
        .map(|(pos,_)| pos)
        .for_each(|&pos| {
            num_active_map.insert(pos, 0);
        });

    // Get all cubes that should increment its active neighbor count
    map.iter()
        .filter(|&(_,ch)| *ch == '#')
        .flat_map(|(p,_)| {
            if part == Part1 {
                p.get_neighbors_3d()
            } else {
                p.get_neighbors_4d()
            }
        })
        .for_each(|pos| {
                if let Some(entry) = num_active_map.get_mut(&pos) {
                    *entry += 1;
                } else {
                    num_active_map.insert(pos, 1);
                }
        });


    // Update map
    for (pos,num_active) in num_active_map.iter() {
        if *num_active == 2 {
            // Skip
            continue;
        }

        if let Some(current_state) = map.get_mut(pos) {
            if *current_state == '.' && *num_active == 3 {
                *current_state = '#';
            } else if *current_state == '#' && !(*num_active == 2 || *num_active == 3) {
                *current_state = '.';
            }
        } else if *num_active == 3 {
            // Inactive, set active
            map.insert(*pos,'#');
        }
    }
}

fn perform_cycles(input:String, part:Part) -> i32 {
    let mut map = parse(input.as_str());

    for _ in 1..=6 {
        mutate(&mut map, part);
    }

    map.values().filter(|&ch| *ch == '#').count() as i32
}

fn part1(input:String) -> i32 {
    perform_cycles(input, Part1)
}


fn part2(input:String) -> i32 {
    perform_cycles(input, Part2)

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test1() {
        let input = ".#.
..#
###";

        assert_eq!(112, part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_17.txt");
        let res = part1(input.to_string());
        assert_eq!(267,res);
    }

    #[test]
    fn test2() {
        let input = ".#.
..#
###";

        assert_eq!(848, part2(input.to_string()));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_17.txt");
        let res = part2(input.to_string());
        assert_eq!(1812,res);
    }

}
