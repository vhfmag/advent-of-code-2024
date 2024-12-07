type Num = usize;

#[derive(Debug, Clone)]
struct Equation {
    result: Num,
    operands: Vec<Num>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

impl Operator {
    fn apply(&self, a: &Num, b: &Num) -> Num {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenation => a * 10usize.pow(b.to_string().len() as u32) + b,
        }
    }
}

fn permutations<T: Clone>(source: &[T], n: u32) -> impl Iterator<Item = Vec<T>> + use<'_, T> {
    let amount = source.len();
    (0..amount.pow(n)).map(move |i| {
        (0..n)
            .map(|j| {
                let shift = amount.pow(j as u32);
                source[(i / shift) % amount].clone()
            })
            .collect()
    })
}

impl Equation {
    fn is_solvable(&self, operators: &[Operator]) -> bool {
        let operands = &self.operands[1..];
        let mut operator_combinations = permutations(operators, operands.len() as u32);

        operator_combinations.any(|operators| {
            self.result
                == std::iter::zip(operands, operators)
                    .fold(self.operands[0], |acc, (operand, op)| {
                        op.apply(&acc, operand)
                    })
        })
    }
}

fn parse_input(input: Option<&str>) -> Vec<Equation> {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day7.input")
    }
    .trim();

    input
        .lines()
        .map(|line| {
            let (result_str, operands_str) = line
                .split_once(": ")
                .expect("Invalid input: couldn't split into result and operands");
            let result = result_str
                .parse()
                .expect("Invalid input: couldn't parse result");

            let operands = operands_str
                .split_whitespace()
                .map(|operand_str| {
                    operand_str
                        .parse()
                        .expect("Invalid input: couldn't parse operand")
                })
                .collect();

            Equation { result, operands }
        })
        .collect()
}

fn sum_solvable_results(input: Option<&str>, operators: &[Operator]) -> Num {
    let equations = parse_input(input);

    equations
        .iter()
        .filter(|eq| eq.is_solvable(operators))
        .map(|eq| eq.result)
        .sum()
}

pub fn part_1(input: Option<&str>) -> Num {
    sum_solvable_results(input, &[Operator::Add, Operator::Multiply])
}

pub fn part_2(input: Option<&str>) -> Num {
    sum_solvable_results(
        input,
        &[Operator::Add, Operator::Multiply, Operator::Concatenation],
    )
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE)), 3749);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(Some(EXAMPLE)), 11387);
    }

    #[test]
    fn permutations() {
        use super::Operator::*;

        assert_eq!(
            super::permutations(&[Add, Multiply, Concatenation], 3).collect::<Vec<_>>(),
            [
                [Add, Add, Add],
                [Multiply, Add, Add],
                [Concatenation, Add, Add],
                [Add, Multiply, Add],
                [Multiply, Multiply, Add],
                [Concatenation, Multiply, Add],
                [Add, Concatenation, Add],
                [Multiply, Concatenation, Add],
                [Concatenation, Concatenation, Add],
                [Add, Add, Multiply],
                [Multiply, Add, Multiply],
                [Concatenation, Add, Multiply],
                [Add, Multiply, Multiply],
                [Multiply, Multiply, Multiply],
                [Concatenation, Multiply, Multiply],
                [Add, Concatenation, Multiply],
                [Multiply, Concatenation, Multiply],
                [Concatenation, Concatenation, Multiply],
                [Add, Add, Concatenation],
                [Multiply, Add, Concatenation],
                [Concatenation, Add, Concatenation],
                [Add, Multiply, Concatenation],
                [Multiply, Multiply, Concatenation],
                [Concatenation, Multiply, Concatenation],
                [Add, Concatenation, Concatenation],
                [Multiply, Concatenation, Concatenation],
                [Concatenation, Concatenation, Concatenation],
            ]
        );
    }
}
