fn count_xmas_at_pos(lines: &Vec<Vec<char>>, (pos_x, pos_y): (usize, usize)) -> usize {
    const DIRECTIONS: &[[(isize, isize); 4]] = &[
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        [(0, 0), (1, 1), (2, 2), (3, 3)],
        [(0, 0), (1, -1), (2, -2), (3, -3)],
    ];

    let pos_x = pos_x as isize;
    let pos_y = pos_y as isize;

    let mut count = 0;
    for direction in DIRECTIONS {
        let substr = direction
            .iter()
            .filter_map(|&(x, y)| {
                let x: usize = (x + pos_x).try_into().ok()?;
                let y: usize = (y + pos_y).try_into().ok()?;

                lines.get(x)?.get(y)
            })
            .collect::<String>();

        if substr == "XMAS" || substr == "SAMX" {
            count += 1;
        }
    }

    count
}

fn check_for_x_mas_in_pos(lines: &Vec<Vec<char>>, (pos_x, pos_y): (usize, usize)) -> bool {
    const DIAGONALS: &[[(usize, usize); 3]] = &[[(0, 0), (1, 1), (2, 2)], [(2, 0), (1, 1), (0, 2)]];

    DIAGONALS.iter().all(|diag| {
        diag.iter()
            .map(|&(x, y)| lines.get(pos_x + x)?.get(pos_y + y))
            .collect::<Option<String>>()
            .map(|s| s == "MAS" || s == "SAM")
            .unwrap_or(false)
    })
}

fn parse_input(input: Option<&str>) -> Vec<Vec<char>> {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day4.input")
    };

    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part_1(input: Option<&str>) -> usize {
    let lines = parse_input(input);

    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            count += count_xmas_at_pos(&lines, (i, j));
        }
    }

    count
}

pub fn part_2(input: Option<&str>) -> usize {
    let lines = parse_input(input);

    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if check_for_x_mas_in_pos(&lines, (i, j)) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE)), 18);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(Some(EXAMPLE)), 9);
    }
}
