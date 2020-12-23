use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.as_str(), 100),
        Part::Part2 => part2(input.as_str())
    };

    result.to_string()
}

#[derive(Debug)]
struct Node {
    label:usize,
    next:usize,
}

impl Node {
    fn new(label:usize,  next:usize) -> Node {
        Node{label, next}
    }
}



fn parse(input:&str) -> (usize, usize, Vec<Node>) {
    let numbers:Vec<usize> = input.chars().filter(|ch| ch.is_ascii_digit()).map(|ch| ch.to_digit(10).unwrap() as usize).collect();
    let mut nodes:Vec<Node> = (0..=numbers.len()).into_iter().map(|label| Node::new(label, 0)).collect();

    // Setup next pointers
    for i in 0..numbers.len() {
        let curr = numbers[i];
        let next = numbers[(i+1) % numbers.len()];
        nodes[curr].next = next;
    }

    //println!("{:?}",nodes);
    (*numbers.first().unwrap(), *numbers.last().unwrap(), nodes)
}




fn play(nodes:&mut Vec<Node>,start:usize, moves:usize) {
    let mut current_index = start;

    for _move in 0..moves {
        // Select cups to move
        let first = nodes[current_index].next;
        let second = nodes[first].next;
        let third = nodes[second].next;

        // Get next destination
        let mut destination = if current_index == 1 {
            nodes.len() - 1
        } else {
            current_index - 1
        };

        while destination == nodes[first].label || destination == nodes[second].label || destination == nodes[third].label {
            if destination == 1 {
                destination = nodes.len() - 1
            } else {
                destination -= 1
            }
        }

        // Move cups
        nodes[current_index].next = nodes[third].next;

        // Destinations next pointer
        let tmp = nodes[destination].next;
        nodes[destination].next = first;
        nodes[third].next = tmp;

        // Move current cup
        current_index = nodes[current_index].next;
    }

}


fn part1(input:&str, moves:usize) -> usize {
    let ( current,_ , mut nodes) = parse(input);
    play(&mut nodes, current, moves);

    let mut result = 0;
    let mut next = 1;
    for _ in 1..nodes.len()-1 {
        next = nodes[next].next;
        result *= 10;
        result += nodes[next].label
    }

    result
}

fn part2(input:&str) -> usize {
    let ( current, last, mut nodes) = parse(input);

    // Extend nodes
    let num_nodes = 1000_000;

    nodes[last].next = nodes.len();

    for n in nodes.len()..=num_nodes {
        if n == num_nodes {
            nodes.push(Node{label:n, next:current});
        } else {
            nodes.push(Node { label: n, next: n + 1 });
        }
    }

    play(&mut nodes, current, 10_000_000);

    let star_one = nodes[1].next;
    let star_two = nodes[star_one].next;

    star_one * star_two
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test1() {
        let input = "389125467";

        assert_eq!(92658374,part1(input,10));
        assert_eq!(67384529,part1(input,100));

    }

    #[test]
    fn test_part1() {
        // 653427918
        let input = include_str!("../../input_23.txt");

        assert_eq!(76952348,part1(input,100));
    }


    #[test]
    fn test2() {
        let input = "389125467";

        assert_eq!(149245887792,part2(input));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_23.txt");

        assert_eq!(72772522064,part2(input));
    }

}
