use super::Part;
use std::collections::{HashMap, HashSet};

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    };

    result.to_string()
}


struct Grid {
    tiles:HashMap<usize,Tile>,
    side:usize,
    corner_ids:Vec<usize>,
    edge_ids:Vec<usize>,
    final_grid:Vec<Vec<char>>,
    state:usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut tiles = HashMap::new();
        input.split("\n\n").map(|s| Tile::parse(s)).for_each(|t| {  tiles.insert(t.number, t); });
        let side_f = tiles.len() as f64;
        let side = side_f.powf(0.5).round() as usize;
        let mut grid = Grid{tiles,side,edge_ids:vec![],corner_ids:vec![],final_grid:vec![vec![]],state:0};

        grid.corner_ids = grid.get_matching_tiles(2);
        grid.edge_ids = grid.get_matching_tiles(3);
        grid
    }

    fn get_tile(&self, id:usize) -> &Tile {
        self.tiles.get(&id).unwrap()
    }

    fn num_matches(&self, tile:&Tile) -> usize {
        self.tiles.values().filter(| t| t.matches(tile)).count()
    }

    fn get_matching_tiles(&self,num:usize) -> Vec<usize> {
        self.tiles.values().filter(|t| self.num_matches(t) == num).map(|t| t.number).collect()
    }

    fn get_monster(&self,offset_x:usize, offset_y:usize) -> HashSet<(usize,usize)> {
        let text = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";
        let mut monster = vec![];
        let mut monster_set = HashSet::new();

        text.lines()
            .for_each(| line | {
                let mut cols = vec![];
                line.chars().for_each(|ch| {
                    cols.push(ch);
                });
                monster.push(cols);
            });


        for y in 0..monster.len() {
            let row = &monster[y];
            for x in 0..row.len() {
                if row[x] == '#' {
                    monster_set.insert((x+offset_x, y+offset_y));
                }
            }
        }

        monster_set
    }

    fn solve(&mut self) -> usize {

        let res = self.solve_int(vec![]).unwrap();
        let tile_side = self.tiles.iter().next().unwrap().1.side;

        // Create final grid
        let mut final_grid = vec![];

        for tile_row in 0..self.side {
            for y in 1..tile_side-1 {
                let mut line = vec![];
                for tile_col in 0..self.side {
                    let tile_index = tile_row * self.side + tile_col;
                    let tile = &res[tile_index];
                    for x in 1..tile_side-1 {
                        line.push(tile.grid[y][x]);
                    }
                }
                final_grid.push(line);
            }
        }

        self.final_grid = final_grid;

        for _ in 0..8 {
            let mut grid_set = self.get_grid_set();
            //self.print_grid();
            let mut monsters_found = 0;
            for y in 0..self.final_grid.len() {
                for x in 0..self.final_grid.len() {
                    let monster_set = self.get_monster(x, y);
                    if monster_set.is_subset(&grid_set) {
                        //println!("Found monster at {},{}", x, y);
                        monsters_found += 1;

                        // Remove monster grid
                        grid_set.retain(|item| !monster_set.contains(item));
                    }
                }
            }

            if monsters_found > 0 {
                //println!("Found {} monsters",monsters_found);
                //println!("Remaining points:{}", grid_set.len());

                return  grid_set.len();
            }
            self.permutate();
        }

        panic!("No solution")
    }

    fn print_grid(&self) {
        println!("---------------------");
        for y in 0..self.final_grid.len() {
            let v = &self.final_grid[y];
            for x in 0..v.len() {
                print!("{}",v[x]);
            }
            println!("");
        }
    }

    fn get_grid_set(&self) -> HashSet<(usize,usize)> {
        let mut grid_set = HashSet::new();
        for y in 0..self.final_grid.len() {
            let row = &self.final_grid[y];
            for x in 0..row.len() {
                if row[x] == '#' {
                    grid_set.insert((x, y));
                }
            }
        }
        grid_set
    }

    fn _flip(&mut self) {
        for y in 0..self.final_grid.len() {
            self.final_grid[y].reverse();
        }
    }

    fn permutate(&mut self) {
        if self.state == 3 || self.state == 7 {
            self._rotate();
            self._flip();
        } else {
            self._rotate();
        }

        self.state += 1;
    }

    fn _rotate(&mut self) {
        let side = self.final_grid.len();
        let mut grid = Vec::new();
        for _ in 0..side {
            let mut v = vec![];
            for _ in 0..side {
                v.push('.')
            }
            grid.push(v);
        }

        for y in 0..self.final_grid.len() {
            for x in 0..self.final_grid[0].len() {
                let x_new = side - y - 1;
                let y_new = x;
                grid[y_new][x_new] = self.final_grid[y][x];
            }
        }

        self.final_grid = grid;
    }


    fn get_next_tiles(&self, acc:&Vec<Tile>) -> Vec<&Tile> {
        let prev_tile = acc.last().unwrap();

        self.tiles.values().filter(|t| /* t.matches(prev_tile) &&*/ acc.iter().find(|p| p.number == t.number).is_none()).collect()
    }


    fn is_corner_index(&self, index:usize) -> bool {
        index == 0 || index == self.side - 1 || index == self.side * (self.side-1) || index == self.side*self.side-1
    }

    fn is_edge_index(&self, index:usize) -> bool {
        !self.is_corner_index(index) && (index < self.side || index % self.side == 0 )
    }


    fn solve_int(&self, result:Vec<Tile>) -> Option<Vec<Tile>> {

        let index = result.len();

        // Verify that accumulated result fits with each other
        if index > 1 {
            for i in index-2..index {
                let current = &result[i];

                // Verify that its a corner tile
                if self.is_corner_index(i) && !self.corner_ids.contains(&current.number){
                    return None;
                } else if self.is_edge_index(i) && !self.edge_ids.contains(&current.number) {
                    return None;
                }

                // Is this a tile that has another tile on its right side?
                if i % self.side != self.side - 1 && i < index-1 {
                    let to_right = &result[i + 1];
                    if !current.fit_to_right(to_right) {
                        return None;
                    }
                }

                // Is this a tile that should have another tile above it?
                if i >= self.side {
                    let above = &result[i - self.side];

                    if !current.fit_to_top(above) {
                        return None;
                    }
                }
            }
        }

        //println!("Solution index:{}, tiles={:?}",index, result.iter().map(|t|t.desc()).collect::<Vec<String>>());

        if index == 0 {
            let corner_tiles = self.get_matching_tiles(2);
            //println!("corner tiles = {:?}", corner_tiles);

            for first_tile_ref in corner_tiles {
                let mut first_tile = self.get_tile(first_tile_ref).clone();
                for _ in 0..8 {
                    if let Some(res) = self.solve_int(vec![first_tile.clone()]) {
                        return Some(res);
                    } else {
                        first_tile.permutate();
                    }
                }
            }
        } else if index < self.tiles.len() {
            let prev_tile = &result[index-1];
            for next_tile_ref in self.get_next_tiles(&result) {
                if index % self.side != 0 && !prev_tile.matches(next_tile_ref) {
                    // Only evaluate the ones that actually matches
                    continue;
                }
                let mut next_tile = next_tile_ref.clone();
                for _ in 0..8 {
                    let mut next_res = result.clone();
                    next_res.push(next_tile.clone());

                    if let Some(res) = self.solve_int( next_res) {
                        return Some(res);
                    } else {
                        next_tile.permutate();
                    }
                }
            }

            return None;
        } else {
            //println!("=====> SOLVED!!!");
        }

        return Some(result);
    }

}

#[derive(Clone,Eq, PartialEq)]
struct Tile {
    number:usize,
    grid:Vec<Vec<char>>,
    side:usize,
    edges:HashSet<Vec<char>>,
    state:usize,
}


impl Tile {
    fn parse(input:&str) -> Tile {
        let mut lines: Vec<&str> = input.lines().collect();
        let header_line = lines.remove(0);
        let number = header_line[5..header_line.len()-1].parse().ok().unwrap();
        let mut grid:Vec<Vec<char>> = Vec::new();
        for y in 0..lines.len() {
            let line = *lines.get(y).unwrap();
            let mut v = vec![];
            for x in 0..line.len() {
                v.push(line.as_bytes()[x] as char);
            }
            grid.push(v);
        }
        let side = grid.len();
        let edges = HashSet::new();
        let mut tile = Tile{number,grid,side,edges:edges,state:0};

        tile.edges.insert(tile.left());
        tile.edges.insert(tile.right());
        tile.edges.insert(tile.top());
        tile.edges.insert(tile.bottom());

        tile.edges.insert(tile.left().iter().rev().copied().collect());
        tile.edges.insert(tile.right().iter().rev().copied().collect());
        tile.edges.insert(tile.top().iter().rev().copied().collect());
        tile.edges.insert(tile.bottom().iter().rev().copied().collect());

        tile
    }

    fn desc(&self) -> String {
        format!("[id:{}, state={}]", self.number, self.state % 8)
    }
    fn left(&self) -> Vec<char> {
        (0..self.side).into_iter().map(|y|self.grid[y][0]).collect()
    }

    fn right(&self) -> Vec<char> {
        (0..self.side).into_iter().map(|y|self.grid[y][self.side-1]).collect()
    }

    fn top(&self) -> Vec<char> {
        self.grid[0].iter().copied().collect()
    }

    fn bottom(&self) -> Vec<char> {
        self.grid[self.side-1].iter().copied().collect()
    }

    fn fit_to_right(&self, other:&Tile) -> bool {
        self.right().eq(&other.left())
    }

    fn fit_to_top(&self, other:&Tile) -> bool {
        self.top().eq(&other.bottom())
    }

    fn matches(&self, other:&Tile) -> bool {
        self.number != other.number && !self.edges.is_disjoint(&other.edges)
    }

    fn flip(&mut self) {
        for y in 0..self.side {
            self.grid[y].reverse();
        }
    }

    fn permutate(&mut self) {
        if self.state == 3 || self.state == 7 {
            self.rotate();
            self.flip();
        } else {
            self.rotate();
        }

        self.state += 1;
    }

    fn rotate(&mut self) {
        let side = self.grid.len();
        let mut grid = Vec::new();
        for _ in 0..side {
            let mut v = vec![];
            for _ in 0..side {
                v.push('.')
            }
            grid.push(v);
        }

        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                let x_new = side - y - 1;
                let y_new = x;
                grid[y_new][x_new] = self.grid[y][x];
            }
        }

        self.grid = grid;
    }

    fn _to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("Tile no:");
        s.push_str(self.number.to_string().as_str());
        s.push('\n');
        self.grid.iter().for_each(|line| {
            line.iter().for_each(|ch| {
                s.push(*ch);
            });
            s.push('\n');
        });
        s
    }
}



fn part1(input:String) -> usize {
    let grid = Grid::new(input.as_str());
    grid.get_matching_tiles(2).iter().product()
}

fn part2(input:String) -> usize {
    let mut grid = Grid::new(input.as_str());
    grid.solve()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = include_str!("../../input_20_test.txt");

        assert_eq!(20899048083289,part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_20.txt");

        assert_eq!(174206308298779,part1(input.to_string()));
    }


    #[test]
    fn test2() {
        let input = include_str!("../../input_20_test.txt");

        assert_eq!(273,part2(input.to_string()));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_20.txt");

        assert_eq!(2409,part2(input.to_string()));
    }

}
