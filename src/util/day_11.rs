use super::Part;
use util::Part::{Part1, Part2};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}

#[derive(Clone)]
struct Seats {
    locations:Vec<char>,
    width:i32,
    height:i32,
    part:Part,
}

impl Seats {
    fn parse(input:&str,part:Part) -> Seats {
        let height = input.lines().count() as i32;
        let width = input.lines().into_iter().next().unwrap().chars().count() as i32;
        let locations:Vec<char> = input.lines().flat_map(|line| line.chars()).collect();

        Seats{locations,width,height,part}
    }

    fn seat_at(&self, x:i32,y:i32) -> Option<char> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            let index = self.width as usize * y as usize + x as usize;
            Some(self.locations[index])
        }
    }

    fn set_seat(&mut self, x:i32,y:i32, ch:char) -> bool {
        let index = self.width as usize * y as usize + x as usize;
        let prev = self.locations[index];
        self.locations[index] = ch;
        prev != ch
    }

    fn _to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.seat_at(x,y).unwrap());
            }
            s.push('\n');
        }
        s
    }

    fn get_total_occupied(&self) -> usize {
        self.locations.iter().filter(|&ch| *ch == '#').count()
    }

    fn get_occupied(&self,x:i32,y:i32) -> usize {
        let directions = vec![(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
        let mut occupied = 0;
        for (dx,dy) in directions {
            let mut next_x = x;
            let mut next_y = y;
            loop {
                next_x += dx;
                next_y += dy;

                if let Some(seat) = self.seat_at(next_x,next_y) {
                    if seat == 'L' {
                        break;
                    } else if seat == '#' {
                        occupied += 1;

                        if occupied > 4 {
                            return occupied;
                        }
                        break;
                    } else if self.part == Part1 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        occupied
    }

    fn get_next_state(&self, next:&mut Seats) -> bool {
        let mut cnt = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let current = self.seat_at(x,y).unwrap();
                let occupied = self.get_occupied(x,y);

                let next_seat_state = if current == 'L' && occupied == 0 {
                    '#'
                } else if current == '#' && (occupied >= 4 && self.part == Part1 || occupied > 4 && self.part == Part2 ) {
                    'L'
                } else {
                    current
                };

                if next.set_seat(x,y,next_seat_state) {
                    cnt += 1
                }
            }
        }

        cnt > 0
    }
}









fn part1(input:String) -> usize {
    let mut seats_a = Seats::parse(input.as_str(),Part1);
    let mut seats_b = seats_a.clone();

    for i in 0..u64::max_value() {
        let updated = match i % 2 {
            0 => seats_a.get_next_state(&mut seats_b),
            1 => seats_b.get_next_state(&mut seats_a),
            _ => panic!("..."),
        };

        if !updated {
            break;
        }
    }

    seats_a.get_total_occupied()
}


fn part2(input:String) -> usize {
    let mut seats_a = Seats::parse(input.as_str(),Part2);
    let mut seats_b = seats_a.clone();

    for i in 0..u64::max_value() {
        let updated = match i % 2 {
            0 => seats_a.get_next_state(&mut seats_b),
            1 => seats_b.get_next_state(&mut seats_a),
            _ => panic!("..."),
        };

        if !updated {
            break;
        }
    }

    seats_a.get_total_occupied()
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
