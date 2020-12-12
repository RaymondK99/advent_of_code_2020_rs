use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}


#[derive(Debug)]
struct Pos {
    x:i32,
    y:i32,
}

impl Pos {
    fn north(&self,n:i32) -> Pos {
        Pos{x:self.x,y:self.y-n}
    }
    fn south(&self,n:i32) -> Pos {
        Pos{x:self.x,y:self.y+n}
    }
    fn east(&self,n:i32) -> Pos {
        Pos{x:self.x+n,y:self.y}
    }
    fn west(&self,n:i32) -> Pos {
        Pos{x:self.x-n,y:self.y}
    }
    fn forward(&self,wp:&Pos,n:i32) -> Pos {
        let x = self.x + wp.x * n;
        let y = self.y + wp.y * n;
        Pos{x,y}
    }

    fn right(&self,degrees:i32) -> Pos {
        match degrees {
            90 => Pos{x:-self.y,y:self.x},
            180 => Pos{x:-self.x,y:-self.y},
            270 => Pos{x:self.y,y:-self.x},
            _ => panic!(),
        }
    }

    fn left(&self,degrees:i32) -> Pos {
        match degrees {
            90 => self.right(270),
            180 => self.right(degrees),
            270 => self.right(90),
            _ => panic!(),
        }
    }
}


fn turn_right(current_dir:char,degrees:i32) -> char {
    let directions = vec!['N','W','S','E'];
    let index = directions.iter().enumerate().find(|&(_,ch)| *ch == current_dir).unwrap().0 as i32;
    let next_index = (index - degrees/90 + 4) % 4;
    directions[next_index as usize]
}

fn turn_left(current_dir:char,degrees:i32) -> char {
    match degrees {
        90 => turn_right(current_dir, 270),
        180 => turn_right(current_dir, 180),
        270 => turn_right(current_dir, 90),
        _ => panic!(),
    }
}

fn parse(input:String) -> Vec<(char,i32)> {
    input.lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().ok().unwrap()))
        .collect()
}

fn part1(input:String) -> i32 {
    let start_pos = Pos{x:0,y:0};
    let (ship_pos,_) = parse(input).iter().fold(
        (start_pos,'E'),|(ship_pos,current_dir),&(ch,n)|{
            let action = match ch {
                'F' => current_dir,
                _ => ch,
            };

            match action {
                'L' => (ship_pos,turn_left(current_dir,n)),
                'R' => (ship_pos,turn_right(current_dir,n)),
                'N' => (ship_pos.north(n),current_dir),
                'S' => (ship_pos.south(n),current_dir),
                'W' => (ship_pos.west(n),current_dir),
                'E' => (ship_pos.east(n),current_dir),
                _ => panic!(".."),
            }
        } );

    ship_pos.x.abs() + ship_pos.y.abs()
}


fn part2(input:String) -> i32 {
    let waypoint_start_pos = Pos{x:10,y:-1};
    let ship_start_pos = Pos{x:0,y:0};

    let (ship_pos,_) = parse(input).iter().fold(
        (ship_start_pos,waypoint_start_pos),|(ship_pos,wp_pos),&(action,n)|{
            match action {
                'N' => (ship_pos,wp_pos.north(n) ),
                'S' => (ship_pos,wp_pos.south(n) ),
                'E' => (ship_pos,wp_pos.east(n) ),
                'W' => (ship_pos,wp_pos.west(n) ),
                'F' => (ship_pos.forward(&wp_pos,n),wp_pos),
                'R' => (ship_pos,wp_pos.right(n)),
                'L' => (ship_pos,wp_pos.left(n)),
                _ => panic!(),
            }
        } );

    ship_pos.x.abs() + ship_pos.y.abs()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test1() {
        let input = "F10
N3
F7
R90
F11";

        assert_eq!(25, part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_12.txt");
        let res = part1(input.to_string());
        assert_eq!(508,res);
    }

    #[test]
    fn test2() {
        let input = "F10
N3
F7
R90
F11";

        assert_eq!(286, part2(input.to_string()));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_12.txt");
        let res = part2(input.to_string());
        assert_eq!(30761,res);
    }

}
