use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_step(&self) -> Pos {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

type PosComp = isize;
type Pos = (PosComp, PosComp);
type GuardState = (Pos, Direction);

#[derive(Clone)]
struct State {
    obstacles: HashSet<Pos>,
    guard: GuardState,
    width: PosComp,
    height: PosComp,
}

impl State {
    fn take_step(&mut self) {
        let step = self.guard.1.get_step();
        let new_guard_pos = (self.guard.0 .0 + step.0, self.guard.0 .1 + step.1);

        if !self.obstacles.contains(&new_guard_pos) {
            self.guard.0 = new_guard_pos;
        } else {
            self.guard.1 = match self.guard.1 {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
        }
    }

    fn is_guard_inside(&self) -> bool {
        self.guard.0 .0 >= 0
            && self.guard.0 .0 < self.width
            && self.guard.0 .1 >= 0
            && self.guard.0 .1 < self.height
    }
}

fn parse_input(input: Option<&str>) -> State {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day6.input")
    }
    .trim();

    let obstacles = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as PosComp, y as PosComp))
                } else {
                    None
                }
            })
        })
        .collect();

    let guard = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(|(x, c)| match c {
                '^' => Some(((x as PosComp, y as PosComp), Direction::Up)),
                'v' => Some(((x as PosComp, y as PosComp), Direction::Down)),
                '<' => Some(((x as PosComp, y as PosComp), Direction::Left)),
                '>' => Some(((x as PosComp, y as PosComp), Direction::Right)),
                _ => None,
            })
        })
        .expect("No guard found");

    State {
        obstacles,
        guard,
        width: input.lines().next().unwrap().len() as PosComp,
        height: input.lines().count() as PosComp,
    }
}

pub fn part_1(input: Option<&str>) -> usize {
    let mut state = parse_input(input);

    let mut visited = HashSet::new();
    while state.is_guard_inside() {
        visited.insert(state.guard);
        state.take_step();
    }

    visited
        .iter()
        .map(|guard| guard.0)
        .collect::<HashSet<_>>()
        .len()
}

pub fn part_2(input: Option<&str>) -> usize {
    // extremely inefficient, but it works ¯\_(ツ)_/¯
    let state = parse_input(input);

    let positions = (0..state.width).flat_map(move |x| (0..state.height).map(move |y| (x, y)));

    let mut count = 0;

    for pos in positions {
        let mut state = state.clone();

        if state.obstacles.contains(&pos) || state.guard.0 == pos {
            continue;
        }

        state.obstacles.insert(pos);

        let mut visited = HashSet::new();
        while state.is_guard_inside() && !visited.contains(&state.guard) {
            visited.insert(state.guard);
            state.take_step();
        }

        if state.is_guard_inside() {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE)), 41);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(Some(EXAMPLE)), 6);
    }
}
