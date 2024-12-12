use std::{
    char,
    collections::{HashMap, HashSet},
};

struct Input {
    regions: Vec<HashSet<Pos>>,
    width: usize,
    height: usize,
}

fn parse_input(input: Option<&str>) -> Input {
    let map = if let Some(input) = input {
        input
    } else {
        include_str!("./day12.input")
    }
    .trim();

    let map: Vec<Vec<_>> = map.lines().map(|x| x.chars().collect()).collect();

    let mut visited = HashSet::new();

    let height = map.len();
    let width = map[0].len();

    let points = (0..height).flat_map(|y| (0..width).map(move |x| (x, y)));

    let regions: Vec<_> = points
        .filter_map(|(x, y)| {
            if visited.contains(&(x, y)) {
                None
            } else {
                let mut region = HashSet::new();
                get_region(&map, (x, y), &mut region);

                visited.extend(region.iter().copied());

                Some(region)
            }
        })
        .collect();

    debug_assert_eq!(regions.iter().flatten().count(), height * width);

    Input {
        regions,
        width,
        height,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

type Pos = (usize, usize);

fn get_region(map: &Vec<Vec<char>>, (x, y): Pos, set: &mut HashSet<Pos>) {
    let char = map[y][x];

    set.insert((x, y));

    DIRECTIONS
        .iter()
        .map(|dir| dir.delta())
        .filter_map(|(dx, dy)| {
            let new_x: usize = ((x as isize) + dx).try_into().ok()?;
            let new_y: usize = ((y as isize) + dy).try_into().ok()?;

            if new_y < map.len() && new_x < map[new_y].len() && map[new_y][new_x] == char {
                Some((new_x, new_y))
            } else {
                None
            }
        })
        .for_each(|pos| {
            if !set.contains(&pos) {
                get_region(map, pos, set);
            }
        });
}

fn get_region_area(region: &HashSet<Pos>) -> usize {
    region.len()
}

fn get_region_boundaries(
    region: &HashSet<Pos>,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (Pos, Direction)> + use<'_> {
    region.iter().flat_map(move |&(x, y)| {
        DIRECTIONS
            .iter()
            .filter(move |dir| {
                let (dx, dy) = dir.delta();
                let new_x = (x as isize) + dx;
                let new_y = (y as isize) + dy;

                if new_x < 0 || new_y < 0 {
                    true
                } else {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;

                    if new_x >= width || new_y >= height {
                        true
                    } else {
                        !region.contains(&(new_x, new_y))
                    }
                }
            })
            .map(move |&delta| ((x, y), delta))
    })
}

pub fn get_region_perimeter(region: &HashSet<Pos>, width: usize, height: usize) -> usize {
    get_region_boundaries(region, width, height).count()
}

fn get_region_sides(region: &HashSet<Pos>, width: usize, height: usize) -> usize {
    let sides_map: HashMap<_, _> = get_region_boundaries(region, width, height)
        .map(|((x, y), delta)| {
            let key = match delta {
                Direction::Up => (y, delta),
                Direction::Down => (y, delta),
                Direction::Left => (x, delta),
                Direction::Right => (x, delta),
            };

            (key, ((x, y), delta))
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert_with(Vec::new).push(value);
            acc
        });

    sides_map
        .into_iter()
        .map(|((_, delta), mut boundaries)| -> usize {
            boundaries.sort_by_key(|&((x, y), _)| match delta {
                Direction::Up => x,
                Direction::Down => x,
                Direction::Left => y,
                Direction::Right => y,
            });

            let discontinuities = boundaries
                .windows(2)
                .filter(|window| {
                    let ((x1, y1), _) = window[0];
                    let ((x2, y2), _) = window[1];

                    let diff = match delta {
                        Direction::Up => x2 - x1,
                        Direction::Down => x2 - x1,
                        Direction::Left => y2 - y1,
                        Direction::Right => y2 - y1,
                    };

                    if diff > 1 {
                        true
                    } else {
                        false
                    }
                })
                .count();

            // there's always at least one side (if it's a single continuous line) + 1 new side per discontinuity
            1 + discontinuities
        })
        .sum()
}

pub fn part_1(input: Option<&str>) -> usize {
    let Input {
        height,
        width,
        regions,
    } = parse_input(input);

    regions
        .iter()
        .map(|region| get_region_area(region) * get_region_perimeter(region, width, height))
        .sum()
}

pub fn part_2(input: Option<&str>) -> usize {
    let Input {
        height,
        width,
        regions,
    } = parse_input(input);

    regions
        .iter()
        .map(|region| get_region_area(region) * get_region_sides(region, width, height))
        .sum()
}

#[cfg(test)]
mod test {
    static EXAMPLE_1: &str = r"
AAAA
BBCD
BBCC
EEEC
";
    static EXAMPLE_2: &str = r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    static EXAMPLE_3: &str = r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
    static EXAMPLE_4: &str = r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
    static EXAMPLE_5: &str = r"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE_1)), 140);
        assert_eq!(super::part_1(Some(EXAMPLE_2)), 772);
        assert_eq!(super::part_1(Some(EXAMPLE_3)), 1930);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(Some(EXAMPLE_1)), 80);
        assert_eq!(super::part_2(Some(EXAMPLE_2)), 436);
        assert_eq!(super::part_2(Some(EXAMPLE_3)), 1206);
        assert_eq!(super::part_2(Some(EXAMPLE_4)), 236);
        assert_eq!(super::part_2(Some(EXAMPLE_5)), 368);
    }
}
