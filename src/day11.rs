use std::collections::HashMap;

type Num = u64;

fn parse_input(input: Option<&str>) -> Vec<Num> {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day11.input")
    }
    .trim();

    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn blink(el: Num) -> Vec<Num> {
    if el == 0 {
        vec![1]
    } else {
        const TEN: Num = 10;
        let digits = el.ilog10() + 1;
        if digits % 2 == 0 {
            let base = TEN.pow(digits / 2);
            let start = el / base;
            let end = el % base;
            vec![start, end]
        } else {
            vec![el * 2024]
        }
    }
}

pub fn part_1(input: Option<&str>, steps: u8) -> usize {
    let input = parse_input(input);
    let mut map: HashMap<_, usize> = input.into_iter().fold(HashMap::new(), |mut map, el| {
        *map.entry(el).or_insert(0) += 1;
        map
    });

    for _ in 0..steps {
        let mut new_map = HashMap::new();

        for (k, v) in map {
            let output = blink(k);
            for el in output {
                *new_map.entry(el).or_insert(0) += v;
            }
        }

        map = new_map;
    }

    map.values().sum()
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
125 17
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE), 25), 55312);
    }

    // no test for part_2 as there's no sample output ¯\_(ツ)_/¯
}
