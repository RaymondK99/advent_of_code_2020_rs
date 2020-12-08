use super::Part;
use util::boot_code::BootCode;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}

fn part1(input:String) -> i32 {
    let mut boot_code = BootCode::parse_text_file(input);
    let (_,acc) = boot_code.run_until_inf_loop_or_finished();
    acc
}

fn part2(input:String) -> i32 {
    let mut boot_code = BootCode::parse_text_file(input);
    loop {
        boot_code.permutate();
        let (finished,acc) = boot_code.run_until_inf_loop_or_finished();

        if finished {
            return  acc;
        } else {
            boot_code.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let res = part1(input.to_string());
        assert_eq!(5,res);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_08.txt");
        let res = part1(input.to_string());
        assert_eq!(1816,res);
    }


    #[test]
    fn test2() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let res = part2(input.to_string());
        assert_eq!(8,res);
    }


    #[test]
    fn test_part2() {
        let input = include_str!("../../input_08.txt");
        let res = part2(input.to_string());
        assert_eq!(1149,res);
    }

}
