use itertools::{iproduct, Itertools};
use std::{
    collections::HashMap,
    ops::{Div, Sub},
};

type PosNum = isize;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(PosNum, PosNum);

impl Sub for &Pos {
    type Output = Pos;
    fn sub(self, other: Self) -> Self::Output {
        Pos(self.0 - other.0, self.1 - other.1)
    }
}

impl Div for Pos {
    type Output = Option<f32>;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0 || rhs.1 == 0 {
            None
        } else {
            let div_x = self.0 as f32 / rhs.0 as f32;
            let div_y = self.1 as f32 / rhs.1 as f32;

            if div_x == div_y {
                Some(div_x)
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<Pos>>,
}

fn parse_input(input: Option<&str>) -> Map {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day8.input")
    }
    .trim();

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| ch.is_alphanumeric())
                .map(move |(x, c)| (c, Pos(x as PosNum, y as PosNum)))
        })
        .fold(
            HashMap::new(),
            |mut antennas: HashMap<char, Vec<Pos>>, (c, pos)| {
                antennas.entry(c).or_default().push(pos);
                antennas
            },
        );

    Map {
        antennas,
        width: input.lines().next().unwrap().len(),
        height: input.lines().count(),
    }
}

fn get_all_positions(width: usize, height: usize) -> impl Iterator<Item = Pos> {
    iproduct!(0..width, 0..height).map(|(x, y)| Pos(x as PosNum, y as PosNum))
}

pub fn part_1(input: Option<&str>) -> usize {
    let map = parse_input(input);

    get_all_positions(map.width, map.height)
        .filter(|pos| {
            map.antennas.values().any(|antenna_positions| {
                antenna_positions
                    .iter()
                    .tuple_combinations()
                    .any(|(a, b)| a - b == pos - a || b - a == pos - b)
            })
        })
        .count()
}

pub fn part_2(input: Option<&str>) -> usize {
    let map = parse_input(input);
    println!("{:?}", map);

    get_all_positions(map.width, map.height)
        .filter(|pos| {
            map.antennas.values().any(|antenna_positions| {
                antenna_positions.iter().tuple_combinations().any(|(a, b)| {
                    ((pos - a) / (a - b))
                        .map(|x| x.fract() == 0.0)
                        .unwrap_or(false)
                        || ((pos - b) / (b - a))
                            .map(|x| x.fract() == 0.0)
                            .unwrap_or(false)
                })
            })
        })
        .count()
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE)), 14);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(Some(EXAMPLE)), 34);
    }
}
