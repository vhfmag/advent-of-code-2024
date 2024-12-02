use std::fmt::Debug;

type Output = isize;

fn parse_input(input: Option<&str>) -> Vec<Vec<Output>> {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day2.input")
    };

    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|el| el.parse().expect("Failed to parse number"))
                .collect()
        })
        .collect()
}

#[derive(Debug)]
enum UnsafeReason {
    NonMonotonic(usize),
    OutOfRangeJump(usize),
}

fn is_report_safe<T: AsRef<[isize]> + Sized>(report: T) -> Option<UnsafeReason> {
    let mut diffs = std::iter::zip(report.as_ref().iter(), (&report.as_ref()[1..]).iter())
        .map(|(a, b)| b - a)
        .peekable();

    let expected_sign = diffs.peek().expect("Got report with 0 levels").signum();

    diffs.enumerate().find_map(|(idx, x)| {
        if x.signum() != expected_sign {
            Some(UnsafeReason::NonMonotonic(idx))
        } else {
            let x = x.abs();
            if x >= 1 && x <= 3 {
                None
            } else {
                Some(UnsafeReason::OutOfRangeJump(idx))
            }
        }
    })
}

pub fn part_1(input: Option<&str>) -> usize {
    parse_input(input)
        .into_iter()
        .filter(|report| is_report_safe(report).is_none())
        .count()
}

fn is_report_safe_without_single_element(report: &Vec<Output>, idx: usize) -> bool {
    let mut report = report.clone();
    report.remove(idx);
    is_report_safe(report).is_none()
}

pub fn part_2(input: Option<&str>) -> usize {
    parse_input(input)
        .into_iter()
        .filter(|report| match is_report_safe(report) {
            Some(UnsafeReason::NonMonotonic(idx)) => {
                is_report_safe(&report[1..]).is_none()
                    || is_report_safe_without_single_element(report, idx)
                    || is_report_safe_without_single_element(report, idx + 1)
            }
            Some(UnsafeReason::OutOfRangeJump(idx)) => {
                is_report_safe_without_single_element(report, idx)
                    || is_report_safe_without_single_element(report, idx + 1)
            }
            None => true,
        })
        .count()
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE)), 2);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(Some(EXAMPLE)), 4);
    }
}
