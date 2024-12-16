use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

type Num = isize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    x: Num,
    y: Num,
}

impl Vec2 {
    fn get_coordinates(&self) -> Num {
        self.y * 100 + self.x
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self {
        let delta = match rhs {
            Direction::Up => Self { x: 0, y: -1 },
            Direction::Down => Self { x: 0, y: 1 },
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Right => Self { x: 1, y: 0 },
        };

        self + delta
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Wall,
    WideWall,
    Block,
    WideBlock,
    Robot,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map(HashMap<Vec2, Item>, Vec2);

impl Map {
    fn to_wide_map(self) -> Self {
        let mut map = HashMap::new();

        for (Vec2 { x, y }, item) in self.0 {
            let wide_item = match item {
                Item::Wall => Item::WideWall,
                Item::Block => Item::WideBlock,
                _ => item,
            };

            map.insert(Vec2 { x: 2 * x, y }, wide_item);
        }

        Self(
            map,
            Vec2 {
                x: 2 * self.1.x,
                y: self.1.y,
            },
        )
    }

    fn get_item_at(&self, pos: Vec2) -> Option<(Vec2, &Item)> {
        match self.0.get(&pos) {
            Some(item) => Some((pos, item)),
            None => {
                let left_pos = Vec2 {
                    x: pos.x - 1,
                    y: pos.y,
                };
                let left_item = self.0.get(&left_pos);

                match left_item {
                    Some(Item::WideWall) | Some(Item::WideBlock) => {
                        left_item.map(|item| (left_pos, item))
                    }
                    _ => None,
                }
            }
        }
    }

    fn unsafe_move_items_at<Positions: IntoIterator<Item = Vec2>>(
        &mut self,
        pos: Positions,
        direction: Direction,
    ) {
        let item_pos_pairs: Vec<_> = pos
            .into_iter()
            .map(|pos| {
                let item = self
                    .0
                    .remove(&pos)
                    .expect("move_item_at called with invalid pos");

                let new_pos = pos + direction;

                (new_pos, item)
            })
            .collect();

        for (new_pos, item) in item_pos_pairs {
            self.0.insert(new_pos, item);
        }
    }

    fn move_item_at(&mut self, pos: Vec2, direction: Direction) -> Option<Vec2> {
        let positions_to_move = self.get_positions_to_move(pos, &direction)?;

        self.unsafe_move_items_at(positions_to_move, direction);

        Some(pos + direction)
    }

    fn get_positions_to_move(
        &self,
        from_pos: Vec2,
        direction: &Direction,
    ) -> Option<HashSet<Vec2>> {
        let to_pos = from_pos + *direction;

        let positions_to_push = match self.get_item_at(from_pos) {
            Some((block_pos, Item::WideBlock)) => match direction {
                Direction::Up | Direction::Down => vec![to_pos, to_pos + Direction::Right],
                Direction::Left => vec![block_pos + Direction::Left],
                Direction::Right => vec![block_pos + Direction::Right + Direction::Right],
                // _ => todo!(),
            },
            _ => vec![to_pos],
        };

        let moved: Vec<_> = positions_to_push
            .into_iter()
            .map(|pos| match self.get_item_at(pos) {
                Some((_, Item::Wall)) | Some((_, Item::WideWall)) => None,
                Some((block_pos, Item::WideBlock)) | Some((block_pos, Item::Block)) => {
                    self.get_positions_to_move(block_pos, direction)
                }
                Some((_, Item::Robot)) => panic!("Invalid map: multiple robots"),
                None => Some(Default::default()),
            })
            .collect();

        if moved.iter().all(|item| item.is_some()) {
            let positions_to_move = moved.into_iter().map(|item| item.expect("Unreachable: found None in `moved` even though .all(|item| item.is_some()) is true")).flatten().chain(std::iter::once(from_pos)).collect();

            Some(positions_to_move)
        } else {
            None
        }
    }
}

impl ToString for Map {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for y in 0..self.1.y {
            for x in 0..self.1.x {
                let pos = Vec2 { x, y };

                let item = match self.get_item_at(pos) {
                    Some((_, Item::Wall)) => '#',
                    Some((_, Item::WideWall)) => '#',
                    Some((_, Item::Block)) => 'O',
                    Some((inner_pos, Item::WideBlock)) => {
                        if inner_pos == pos {
                            '['
                        } else {
                            ']'
                        }
                    }
                    Some((_, Item::Robot)) => '@',
                    None => '.',
                };

                result.push(item);
            }

            result.push('\n');
        }

        // result.trim().to_string()
        result
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    map: Map,
    directions: Vec<Direction>,
}

fn parse_map(input: &str) -> Map {
    Map(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    let pos = Vec2 {
                        x: x as Num,
                        y: y as Num,
                    };

                    let item = match c {
                        '#' => Item::Wall,
                        'O' => Item::Block,
                        '@' => Item::Robot,
                        '.' => return None,
                        _ => panic!("Invalid item"),
                    };

                    Some((pos, item))
                })
            })
            .collect(),
        Vec2 {
            x: input.lines().next().unwrap().len() as Num,
            y: input.lines().count() as Num,
        },
    )
}

fn parse_input(input: Option<&str>) -> Input {
    let input = input.unwrap_or_else(|| include_str!("day15.input")).trim();

    let (map_str, directions_str) = input.split_once("\n\n").expect("Invalid input");

    let map: Map = parse_map(map_str);

    let directions = directions_str
        .split("\n")
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect();

    Input { map, directions }
}

fn move_robot(map: &mut Map, directions: &Vec<Direction>) -> Vec2 {
    let mut robot_pos = map
        .0
        .iter()
        .find_map(|(pos, item)| {
            if item == &Item::Robot {
                Some(*pos)
            } else {
                None
            }
        })
        .expect("Invalid map: no robot");

    for &direction in directions.into_iter() {
        if let Some(new_pos) = map.move_item_at(robot_pos, direction) {
            robot_pos = new_pos;
        }
    }

    robot_pos
}

pub fn part_1(input: Option<&str>) -> Num {
    let Input {
        mut map,
        directions,
    } = parse_input(input);

    move_robot(&mut map, &directions);

    map.0
        .iter()
        .filter(|(_, item)| item == &&Item::Block)
        .map(|(pos, _)| pos.get_coordinates())
        .sum()
}

pub fn part_2(input: Option<&str>) -> Num {
    let Input { map, directions } = parse_input(input);

    let mut map = map.to_wide_map();

    move_robot(&mut map, &directions);

    map.0
        .iter()
        .filter(|(_, item)| item == &&Item::Block || item == &&Item::WideBlock)
        .map(|(pos, _)| pos.get_coordinates())
        .sum()
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
  ";

    #[test]
    fn part_1() {
        let super::Input {
            mut map,
            directions,
        } = super::parse_input(Some(EXAMPLE));

        super::move_robot(&mut map, &directions);

        assert_eq!(
            map,
            super::parse_map(
                r"
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
"
                .trim()
            )
        );

        assert_eq!(super::part_1(Some(EXAMPLE)), 10092);
    }

    #[test]
    fn part_2() {
        const EXPECTED: &str = r"
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
";

        let super::Input { map, directions } = super::parse_input(Some(EXAMPLE));

        let mut map = map.to_wide_map();

        super::move_robot(&mut map, &directions);

        assert_eq!(
            map.to_string(),
            EXPECTED.trim(),
            "invalid map output\nours:\t\treference:\n{}",
            std::iter::zip(map.to_string().lines(), EXPECTED.trim().lines())
                .map(|(before, after)| format!("{}\t{}", before, after))
                .collect::<Vec<_>>()
                .join("\n")
        );

        assert_eq!(super::part_2(Some(EXAMPLE)), 9021);
    }
}
