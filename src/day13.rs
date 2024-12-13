use nalgebra::{matrix, vector};
use regex::Regex;

type Num = isize;
type F = f64;

struct Pos {
    x: Num,
    y: Num,
}

struct Solution {
    a: Num,
    b: Num,
}

struct Machine {
    button_a: Pos,
    button_b: Pos,
    prize: Pos,
}

fn parse_button_line(line: &str, button_regex: &Regex) -> Pos {
    let captures = button_regex.captures(line).expect("Invalid button line");

    let x = captures["x"].parse().expect("Invalid button x");
    let y = captures["y"].parse().expect("Invalid button y");

    Pos { x, y }
}

fn parse_machines(input: Option<&str>) -> Vec<Machine> {
    let button_regex =
        Regex::new(r"Button .*?: X(?<x>[-+]?\d+), Y(?<y>[-+]?\d+)").expect("Invalid button regex");
    let prize_regex = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").expect("Invalid prize regex");
    let input = input
        .unwrap_or_else(|| include_str!("./day13.input"))
        .trim();

    input
        .split("\n\n")
        .map(|machine_str| {
            let [a_str, b_str, prize_str] = machine_str
                .lines()
                .collect::<Vec<_>>()
                .try_into()
                .expect("expected 3 lines");

            let button_a = parse_button_line(a_str, &button_regex);
            let button_b = parse_button_line(b_str, &button_regex);
            let prize = {
                let captures = prize_regex.captures(prize_str).expect("Invalid prize line");

                let x = captures["x"].parse().expect("Invalid prize x");
                let y = captures["y"].parse().expect("Invalid prize y");

                Pos { x, y }
            };

            Machine {
                button_a,
                button_b,
                prize,
            }
        })
        .collect()
}

fn into_approx_int(f: F) -> Option<F> {
    const EPSILON: F = 1e-4;

    let rounded = f.round();
    if (f - rounded).abs() < EPSILON {
        Some(rounded)
    } else {
        None
    }
}

fn solve_machine(machine: &Machine) -> Option<Solution> {
    let Machine {
        button_a,
        button_b,
        prize: target,
    } = &machine;

    let solution = matrix![
        button_a.x as F, button_b.x as F;
        button_a.y as F, button_b.y as F
    ]
    .lu()
    .solve(&vector![target.x as F, target.y as F])?;

    let a = into_approx_int(solution[0])? as Num;
    let b = into_approx_int(solution[1])? as Num;

    Some(Solution { a, b })
}

fn solution_cost(solution: Solution) -> Num {
    const A_COST: Num = 3;
    const B_COST: Num = 1;

    solution.a * A_COST + solution.b * B_COST
}

pub fn part_1(input: Option<&str>) -> Num {
    let machines = parse_machines(input);

    machines
        .into_iter()
        .filter_map(|machine| solve_machine(&machine))
        .map(solution_cost)
        .sum()
}

pub fn part_2(input: Option<&str>) -> Num {
    let machines = parse_machines(input);

    machines
        .into_iter()
        .filter_map(|mut machine| {
            const PART_2_OFFSET: Num = 10_000_000_000_000;
            machine.prize.x += PART_2_OFFSET;
            machine.prize.y += PART_2_OFFSET;

            solve_machine(&machine)
        })
        .map(solution_cost)
        .sum()
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
    ";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE)), 480);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(None), 93209116744825);
    }
}
