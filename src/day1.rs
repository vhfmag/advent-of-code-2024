use std::collections::HashMap;

type Output = isize;

fn parse_day_1(input: Option<&str>) -> (Vec<Output>, Vec<Output>) {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day1.input")
    };

    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_once(" ")
                .map(|(a, b)| -> (Output, Output) {
                    (
                        a.trim().parse().expect("Failed to parse number"),
                        b.trim().parse().expect("Failed to parse number"),
                    )
                })
                .expect("Failed to parse line")
        })
        .collect()
}

pub fn day_1_part_1(input: Option<&str>) -> Output {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = parse_day_1(input);

    list1.sort();
    list2.sort();

    std::iter::zip(list1, list2)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn day_1_part_2(input: Option<&str>) -> Output {
    let (list1, list2): (Vec<_>, Vec<_>) = parse_day_1(input);

    let count_map = list2.into_iter().fold(HashMap::new(), |mut map, x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });

    list1
        .into_iter()
        .map(|x| x * count_map.get(&x).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_day_1_part_1() {
        let result = day_1_part_1(Some(EXAMPLE));

        assert_eq!(result, 11);
    }

    #[test]
    fn test_day_1_part_2() {
        let result = day_1_part_2(Some(EXAMPLE));

        assert_eq!(result, 31);
    }
}
