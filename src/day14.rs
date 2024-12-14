use std::{
    collections::HashMap,
    io::{stdin, Read},
    num::ParseIntError,
    ops::{Add, Mul},
    str::FromStr,
};

type Num = isize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    x: Num,
    y: Num,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Vec2 {
    fn normalize(&mut self, size: &Vec2) {
        self.x %= size.x;
        self.y %= size.y;

        if self.x < 0 {
            self.x += size.x;
        }

        if self.y < 0 {
            self.y += size.y;
        }
    }

    fn quadrant(&self, size: &Vec2) -> Option<Quadrant> {
        let x_condition: i8 = if self.x < size.x / 2 {
            -1
        } else if self.x > size.x / 2 {
            1
        } else {
            0
        };

        let y_condition: i8 = if self.y < size.y / 2 {
            -1
        } else if self.y > size.y / 2 {
            1
        } else {
            0
        };

        match (x_condition, y_condition) {
            (-1, -1) => Some(Quadrant::TopLeft),
            (1, -1) => Some(Quadrant::TopRight),
            (-1, 1) => Some(Quadrant::BottomLeft),
            (1, 1) => Some(Quadrant::BottomRight),
            _ => None,
        }
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

impl Mul<Num> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Num) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl FromStr for Vec2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y] = s
            .split(",")
            .map(|part| part.parse())
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap();

        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

fn parse_input(input: Option<&str>) -> Vec<Robot> {
    let input = input.unwrap_or_else(|| include_str!("day14.input")).trim();

    input
        .lines()
        .map(|line| {
            let [pos, vel] = line
                .split(" ")
                .map(|part| part.split("=").last().unwrap().parse().unwrap())
                .collect::<Vec<Vec2>>()
                .try_into()
                .unwrap();

            Robot { pos, vel }
        })
        .collect()
}

fn print_map(robots: &Vec<Robot>, size: &Vec2) {
    for y in 0..size.y {
        for x in 0..size.x {
            let pos = Vec2 { x, y };
            let robot_count = robots.iter().filter(|r| r.pos == pos).count();

            match robot_count {
                0 => print!("."),
                x => print!("{}", x),
            };
        }

        println!();
    }
}

const ITERATIONS: usize = 100;
pub fn part_1(input: Option<&str>, size: (Num, Num)) -> Num {
    let mut robots = parse_input(input);
    let size = Vec2 {
        x: size.0,
        y: size.1,
    };

    for robot in &mut robots {
        robot.pos = robot.pos + robot.vel * ITERATIONS as Num;
        robot.pos.normalize(&size);
    }

    #[cfg(debug_assertions)]
    print_map(&robots, &size);

    let robots_per_quadrant = robots.into_iter().fold(
        HashMap::from([
            (Quadrant::TopLeft, 0),
            (Quadrant::TopRight, 0),
            (Quadrant::BottomLeft, 0),
            (Quadrant::BottomRight, 0),
        ]),
        |mut acc, robot| {
            if let Some(quadrant) = robot.pos.quadrant(&size) {
                *acc.get_mut(&quadrant).unwrap() += 1;
            }

            acc
        },
    );

    robots_per_quadrant.values().product()
}

fn has_square_of_size(robots: &Vec<Robot>, board_size: &Vec2, square_size: Num) -> bool {
    let square = (0..square_size)
        .flat_map(|y| {
            (0..square_size).map(move |x| Vec2 {
                x: x as Num,
                y: y as Num,
            })
        })
        .collect::<Vec<_>>();

    for x in 0..(board_size.x - square_size as Num) {
        for y in 0..(board_size.y - square_size as Num) {
            let base_pos = Vec2 { x, y };

            if square.iter().all(|pos| {
                let checked_pos = base_pos + *pos;
                robots.iter().any(|robot| robot.pos == checked_pos)
            }) {
                return true;
            }
        }
    }

    false
}

pub fn part_2(input: Option<&str>, size: (Num, Num)) -> Num {
    let mut robots = parse_input(input);
    let size = Vec2 {
        x: size.0,
        y: size.1,
    };

    let mut seconds = 0;
    loop {
        for robot in &mut robots {
            robot.pos = robot.pos + robot.vel;
            robot.pos.normalize(&size);
        }

        seconds += 1;

        if has_square_of_size(&robots, &size, 3) {
            break;
        }
    }

    print_map(&robots, &size);

    seconds
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE), (11, 7)), 12);
    }
}
