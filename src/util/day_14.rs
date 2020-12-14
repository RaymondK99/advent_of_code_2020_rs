use super::Part;
use std::collections::{HashMap};


pub fn solve(input : String, part: Part) -> String {
    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}

fn convert_mask(mask:&str,set_x:bool)  -> String {
    mask.chars().map(|ch| {
        if set_x && ch == 'X' {
            '1'
        } else if !set_x && ch == 'X' {
            '0'
        } else {
            ch
        }
    }).collect()
}


fn expand_float_address(address:&[u8], acc_addr:u64, value:u64, memory:&mut HashMap<u64,u64>) {
    let bit_no = address.len();

    if address.is_empty() {
        // Update memory, terminate recursion
        memory.insert(acc_addr, value);
    } else if address[0] == b'X' {
        let next_acc_addr = acc_addr | 1 << (bit_no-1);
        expand_float_address(&address[1..], acc_addr, value, memory);
        expand_float_address(&address[1..], next_acc_addr, value, memory);
    } else {
        let bit  = match address[0] {
            b'0' => 0,
            b'1' => 1,
            _ => panic!("..."),
        };

        let next_acc_addr = acc_addr | bit << (bit_no-1);
        expand_float_address(&address[1..], next_acc_addr, value, memory);
    }
}



fn update_memory(mask:&str, address_str:&str, value:u64, memory:&mut HashMap<u64,u64>) {
    let address = u64::from_str_radix(address_str,10).unwrap();
    let float_address:String = mask.chars().enumerate().map(|(index,mask_bit)| {
        match mask_bit {
            '0' => if address & (1 << (mask.len()-index-1)) != 0 {
                '1'
            } else {
                '0'
            }
            _ => mask_bit,
        }
    }).collect();

    expand_float_address(float_address.as_bytes(), 0, value, memory)
}

fn part1(input:String) -> u64 {
    let mut memory:HashMap<u32,u64> = HashMap::new();
    let mut mask_ones = 0;
    let mut mask_zeroes = 0;
    input.lines().for_each(|line| {
        let mut it = line.split('=');
        let instr_type = it.next().unwrap().trim();
        let value_str = it.next().unwrap().trim();
        if instr_type.starts_with("mask") {
            mask_zeroes = u64::from_str_radix(convert_mask(value_str, true).as_str(), 2).unwrap();
            mask_ones = u64::from_str_radix(convert_mask(value_str, false).as_str(), 2).unwrap();
        } else {
            let address:u32 = instr_type[4..instr_type.len()-1].parse().ok().unwrap();
            let value = u64::from_str_radix(value_str, 10).unwrap();
            memory.insert(address, value & mask_zeroes | mask_ones);
        }
    });

    memory.values().copied().sum()
}


fn part2(input:String) -> u64 {
    let mut memory:HashMap<u64,u64> = HashMap::with_capacity(100_000);
    let mut mask_str = String::new();
    input.lines().for_each(|line| {
        let mut it = line.split('=');
        let instr_type = it.next().unwrap().trim();
        let value_str = it.next().unwrap().trim();
        if instr_type.starts_with("mask") {
            mask_str = value_str.to_string();
        } else {
            let address = &instr_type[4..instr_type.len()-1];
            let value = u64::from_str_radix(value_str, 10).unwrap();
            update_memory(mask_str.as_str(), address, value, &mut memory);
        }
    });

    memory.values().copied().sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        assert_eq!(165, part1(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_14.txt");
        let res = part1(input.to_string());
        assert_eq!(5875750429995,res);
    }

    #[test]
    fn test2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(208, part2(input.to_string()));
    }




    #[test]
    fn test_part2() {
        let input = include_str!("../../input_14.txt");
        let res = part2(input.to_string());
        assert_eq!(5272149590143,res);
    }

}
