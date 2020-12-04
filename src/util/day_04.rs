use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    };

    result.to_string()
}

struct Passport {
    fields:Vec<(String,String)>,
}

impl Passport {
    fn parse(input:&str) -> Passport {
        let fields = input.split(|ch| ch == '\n' || ch == ' ').
            map(|field| {
                let pair: Vec<&str> = field.split(':').collect();
                (pair[0].to_string(),pair[1].to_string())
            }).collect();

        Passport{fields}
    }

    fn has_required_fields(&self) -> bool {
        let keys = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];
        keys.iter()
            .map(|key| self.fields.iter().any(|(k,_)| k.eq(key)))
            .all(|a| a )
    }

    fn numeric_range(value:&str, min:i32, max:i32) -> bool {
        match value.parse() {
            Ok(value) => min <= value && max >= value,
            _ => false,
        }
    }

    fn validate_fields(&self) -> bool {
        self.fields.iter().map(|(key,value)| {
            match key.as_str() {
                "byr" => Passport::numeric_range(value, 1920, 2002),
                "iyr" => Passport::numeric_range(value, 2010, 2020),
                "eyr" => Passport::numeric_range(value, 2020, 2030),
                "ecl" => ["amb","blu","brn","gry","grn","hzl","oth"].contains(&value.as_str()),
                "hgt" => (value.ends_with("cm") && value.len() == 5 && Passport::numeric_range(&value[0..3], 150, 193)) ||
                    (value.ends_with("in") && value.len() == 4 && Passport::numeric_range(&value[0..2], 59, 76)),
                "hcl" => value.len() == 7 && value.starts_with('#') && value.chars().filter(|c|c.is_ascii_hexdigit()).count() == 6,
                "pid" => value.len() == 9 && value.parse::<u64>().is_ok(),
                "cid" => true,
                _ => false
            }
        }).all(|a|a)
    }
}



fn part1(input:String) -> usize {
    input.split("\n\n")
        .map(|item| Passport::parse(item) )
        .filter( |passport| passport.has_required_fields())
        .count()
}

fn part2(input:String) -> usize {
    input.split("\n\n")
        .map(|item| Passport::parse(item) )
        .filter( |passport| passport.has_required_fields())
        .filter(|passport| passport.validate_fields())
        .count()
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!("2", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {

        let input = include_str!("../../input_04.txt");

        assert_eq!("264", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let valids = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";


        assert_eq!("4", solve(valids.to_string(), Part2));


        let invalids = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";


        assert_eq!("0", solve(invalids.to_string(), Part2));
    }


    #[test]
    fn test_part2() {

        let input = include_str!("../../input_04.txt");

        assert_eq!("224", solve(input.to_string(), Part2));
    }

}
