use std::collections::HashSet;

type Num = u32;
type Map = Vec<Vec<Num>>;

fn parse_input(input: Option<&str>) -> Map {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day10.input")
    }
    .trim();

    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid input: couldn't parse size") as Num)
                .collect()
        })
        .collect()
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn find_trail(map: &Map, x: usize, y: usize) -> Vec<(usize, usize)> {
    let current_value = map[x][y];

    if current_value == 9 {
        return vec![(x, y)];
    }

    DIRECTIONS
        .iter()
        .filter_map(|(dx, dy)| {
            let new_x: usize = (x as isize + dx).try_into().ok()?;
            let new_y: usize = (y as isize + dy).try_into().ok()?;

            if new_x >= map.len()
                || new_y >= map[new_x].len()
                || map[new_x][new_y] != current_value + 1
            {
                None
            } else {
                Some(find_trail(map, new_x, new_y))
            }
        })
        .flatten()
        .collect()
}

pub fn part_1(input: Option<&str>) -> usize {
    let map = parse_input(input);

    let zeroes = map.iter().enumerate().flat_map(|(i, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, &n)| n == 0)
            .map(move |(j, _)| (i, j))
    });

    zeroes
        .flat_map(|(x, y)| {
            find_trail(&map, x, y)
                .into_iter()
                .map(move |end| ((x, y), end))
        })
        .collect::<HashSet<_>>()
        .len()
}

pub fn part_2(input: Option<&str>) -> usize {
    let map = parse_input(input);

    let zeroes = map.iter().enumerate().flat_map(|(i, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, &n)| n == 0)
            .map(move |(j, _)| (i, j))
    });

    zeroes
        .map(|(x, y)| find_trail(&map, x, y).into_iter().count())
        .sum()
}

#[cfg(test)]
mod test {
    static EXAMPLE_1: &str = r"
0123
1234
8765
9876
";

    static EXAMPLE_2: &str = r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE_1)), 1);
        assert_eq!(super::part_1(Some(EXAMPLE_2)), 36);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(Some(EXAMPLE_2)), 81);
    }
}
