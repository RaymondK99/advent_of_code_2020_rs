use super::Part;
use std::collections::HashMap;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}


fn _map_as_str(map:&HashMap<(i32, i32),char>) -> String {
    let max_y = map.keys().map(|&(_,y)| y).max().unwrap();
    let max_x = map.keys().map(|&(x,_)| x).max().unwrap();
    let mut s = String::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            s.push(*map.get(&(x,y)).unwrap());
        }
        s.push('\n');
    }

    s
}

fn next_state(current:&HashMap<(i32,i32),char>,
              next:&mut HashMap<(i32,i32),char>,
              next_seat_state:fn(&HashMap<(i32,i32),char>,i32,i32) -> char ) -> bool {
    let mut cnt = 0;
    current.iter().for_each( |((x,y),ch)| {
       let next_ch = next_seat_state(&current, *x,*y);
        next.insert((*x, *y), next_ch);

        if next_ch != *ch {
            cnt += 1;
        }
    });

    cnt > 0
}

fn get_next_seat_state_adjacent(map:&HashMap<(i32, i32),char>, x:i32, y:i32) -> char {
    let occupied = get_adjacent_seats(&map, x, y).iter().filter(|&ch| *ch == '#').count();
    let current = *map.get(&(x,y)).unwrap();
    if current == 'L' && occupied == 0 {
        '#'
    } else if current == '#' && occupied >= 4 {
        'L'
    } else {
        current
    }
}


fn get_next_seat_state_visible(map:&HashMap<(i32, i32),char>, x:i32, y:i32) -> char {
    let directions = vec![(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

    let occupied = directions.iter().map( |&(dx,dy)| visible_seat(&map, x,y,dx,dy))
        .filter(|item| item.is_some())
        .filter(|item| item.unwrap() == '#').count();


    let current = *map.get(&(x,y)).unwrap();
    if current == 'L' && occupied == 0 {
        '#'
    } else if current == '#' && occupied > 4 {
        'L'
    } else {
        current
    }
}

fn visible_seat(map:&HashMap<(i32, i32),char>,x:i32, y:i32,dx:i32, dy:i32) -> Option<char> {
    let mut pos_x = x;
    let mut pos_y = y;
    loop {
        pos_y += dy;
        pos_x += dx;

        if let Some(ch) = map.get(&(pos_x,pos_y)) {
            if *ch == '.' {
                continue;
            } else {
                return Some(*ch)
            }
        } else {
            return None
        }
    }
}


fn get_adjacent_seats(map:&HashMap<(i32, i32),char>, x:i32, y:i32) -> Vec<char> {
    let positions = vec![(x-1,y-1),(x-1,y),(x-1,y+1),(x,y-1),(x,y+1),(x+1,y-1),(x+1,y),(x+1,y+1)];

    positions.iter()
        .map(|&(x,y)| map.get(&(x,y)))
        .filter( |item| item.is_some())
        .map(|item| *item.unwrap())
        .collect()
}

fn parse(input:&String) -> HashMap<(i32,i32),char> {
    let mut map:HashMap<(i32,i32),char> = HashMap::new();

    input.lines().enumerate()
        .flat_map( |(y,line)| line.chars().enumerate().map(move |(x,ch)| (x,y,ch)))
        .for_each(|(x,y,ch)| {map.insert((x as i32,y as i32),ch);});
    map
}


fn part1(input:String) -> usize {
    let mut map_a = parse(&input);
    let mut map_b = map_a.clone();

    for i in 0..u64::max_value() {
        let updated = match i % 2 {
            0 => next_state(&map_a, &mut map_b, get_next_seat_state_adjacent),
            1 => next_state(&map_b, &mut map_a, get_next_seat_state_adjacent),
            _ => panic!("..."),
        };

        if !updated {
            break;
        }
    }

    map_a.values().filter(|&ch| *ch == '#').count()

}


fn part2(input:String) -> usize {
    let mut map_a = parse(&input);
    let mut map_b = map_a.clone();

    for i in 0..u64::max_value() {
        let updated = match i % 2 {
            0 => next_state(&map_a, &mut map_b,get_next_seat_state_visible),
            1 => next_state(&map_b, &mut map_a,get_next_seat_state_visible),
            _ => panic!("..."),
        };

        if !updated {
            break;
        }
    }

    map_a.values().filter(|&ch| *ch == '#').count()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let res = part1(input.to_string());
        println!("{}",res);
        assert_eq!(37,res);
    }


    #[test]
    fn test2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let res = part2(input.to_string());
        println!("{}",res);
        assert_eq!(26,res);
    }


}
