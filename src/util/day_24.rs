use super::Part;
use std::collections::{HashMap};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    };

    result.to_string()
}

#[derive(Debug,Hash,Eq, PartialEq, Clone)]
struct Pos{
    x:i32,
    y:i32,
}

impl Pos {

    fn neighbors(&self) -> Vec<Pos> {
        vec![self.east(), self.west(), self.north_east(), self.north_west(), self.south_west(), self.south_east()]
    }

    fn east(&self) -> Pos {
        Pos{x:self.x+1,y:self.y}
    }

    fn west(&self) -> Pos {
        Pos{x:self.x-1,y:self.y}
    }

    fn south_west(&self) -> Pos {
        Pos{x:self.x-1,y:self.y+1}
    }

    fn north_west(&self) -> Pos {
        Pos{x:self.x,y:self.y-1}
    }

    fn north_east(&self) -> Pos {
        Pos{x:self.x+1,y:self.y-1}
    }

    fn south_east(&self) -> Pos {
        Pos{x:self.x,y:self.y+1}
    }
}

fn parse(input:&str, acc:Pos) -> Pos {
    if input.starts_with("e") {
        return parse(&input[1..], acc.east());
    } else if input.starts_with("w") {
        return parse(&input[1..], acc.west());
    } else if input.starts_with("nw") {
        return parse(&input[2..], acc.north_west());
    } else if input.starts_with("sw") {
        return parse(&input[2..], acc.south_west());
    } else if input.starts_with("ne") {
        return parse(&input[2..], acc.north_east() );
    } else if input.starts_with("se") {
        return parse(&input[2..], acc.south_east());
    } else if input.is_empty() {
        acc
    } else {
        panic!("..");
    }
}

fn build_floor(input:&str) -> HashMap<Pos,char> {
    let positions:Vec<Pos> = input.lines().map(|line| parse(line,Pos{y:0,x:0})).collect();
    let mut map = HashMap::new();
    for pos in positions {
        if let Some (color) =  map.get(&pos) {
            if *color == 'b' {
                map.insert(pos, 'w');
            } else if *color == 'w' {
                map.insert(pos,'b');
            }
        } else {
            map.insert(pos,'b');
        }
    }
    map
}

fn mutate(map:&mut HashMap<Pos,char>) {
    let mut m = HashMap::new();

    // Insert initial 0 count for all black tiles
    map.iter().filter(|(_,&v)|v == 'b').for_each(|(k,_)| {
        m.insert(k.clone(), 0);
    });

    // Add a count for all black tiles neighbors
    map.iter().filter(|(_,&v)|v == 'b').for_each(| (pos, _) | {
        for n in pos.neighbors().iter() {
            if let Some(count) = m.get_mut(n) {
                *count += 1;
            } else {
                m.insert(n.clone(),1);
            }
        }
    });


    m.iter().for_each(|(pos, count)| {
       if let Some(color) = map.get_mut(pos) {
           if *color == 'b' && ( *count == 0 || *count > 2) {
               *color = 'w';
           } else if *color == 'w' && *count == 2 {
               *color = 'b';
           }
       } else {
           // Unknown tile is white, flip it to black
           if *count == 2 {
               map.insert(pos.clone(), 'b');
           }
       }
    });
}

fn part1(input:String) -> usize {
    build_floor(input.as_str()).values().filter(|&c| *c == 'b').count()
}

fn part2(input:String) -> usize {
    let mut map= build_floor(input.as_str());
    for _day in 1..=100 {
        mutate(&mut map);
    }

    map.values().filter(|&c| *c == 'b').count()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        assert_eq!(10,part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_24.txt");

        assert_eq!(230,part1(input.to_string()));
    }


    #[test]
    fn test2() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        assert_eq!(2208,part2(input.to_string()));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_24.txt");

        assert_eq!(3565,part2(input.to_string()));
    }

}
