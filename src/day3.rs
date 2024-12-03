use regex::Regex;

pub fn part_1(input: Option<&str>) -> usize {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day3.input")
    };

    let re = Regex::new(r"mul\((?P<a>\d{1,3}),(?P<b>\d{1,3})\)").expect("invalid regex");

    re.captures_iter(input)
        .map(|cap| {
            let a = cap["a"].parse::<usize>().expect("invalid number");
            let b = cap["b"].parse::<usize>().expect("invalid number");
            a * b
        })
        .sum()
}

pub fn part_2(input: Option<&str>) -> usize {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day3.input")
    };

    let re = Regex::new(r"mul\((?P<a>\d{1,3}),(?P<b>\d{1,3})\)|do\(\)|don't\(\)")
        .expect("invalid regex");

    let mut are_instructions_enabled = true;
    re.captures_iter(input)
        .map(|cap| {
            if &cap[0] == "do()" {
                are_instructions_enabled = true;
                return 0;
            } else if &cap[0] == "don't()" {
                are_instructions_enabled = false;
                return 0;
            } else if are_instructions_enabled {
                let a = cap["a"].parse::<usize>().expect("invalid number");
                let b = cap["b"].parse::<usize>().expect("invalid number");
                a * b
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE)), 161);
    }

    // #[test]
    // fn part_2() {
    //     assert_eq!(super::part_2(Some(EXAMPLE)), 4);
    // }
}
