use std::{cmp::Ordering, collections::HashMap};

type Num = u8;

struct Output {
    before_map: HashMap<Num, Vec<Num>>,
    page_updates: Vec<Vec<Num>>,
}

fn parse_input(input: Option<&str>) -> Output {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day5.input")
    }
    .trim();

    let (spec_str, update_str) = input
        .split_once("\n\n")
        .expect("Invalid input: couldn't split into sections");

    let spec = spec_str.lines().map(|line| {
        let (before, after) = line
            .split_once("|")
            .expect("Invalid input: couldn't split into before and after");
        let before = before
            .parse()
            .expect("Invalid input: couldn't parse before");
        let after = after.parse().expect("Invalid input: couldn't parse after");
        (before, after)
    });

    let before_map = spec.fold(HashMap::new(), |mut before_map, (before, after)| {
        before_map
            .entry(before)
            .or_insert_with(Vec::new)
            .push(after);
        before_map
    });

    let page_updates = update_str
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().expect("Invalid input: couldn't parse number"))
                .collect()
        })
        .collect();

    Output {
        before_map,
        page_updates,
    }
}

fn get_page_updates_cmp(
    before_map: &HashMap<Num, Vec<Num>>,
) -> impl Fn(&Num, &Num) -> Ordering + use<'_> {
    move |a, b| {
        if before_map
            .get(a)
            .map(|before_list| before_list.contains(b))
            .unwrap_or(false)
        {
            return Ordering::Greater;
        }

        if before_map
            .get(b)
            .map(|after_list| after_list.contains(a))
            .unwrap_or(false)
        {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

pub fn part_1(input: Option<&str>) -> u16 {
    let Output {
        before_map,
        page_updates,
    } = parse_input(input);

    let page_updates_cmp = get_page_updates_cmp(&before_map);

    page_updates
        .iter()
        .filter(|page_update| page_update.is_sorted_by(|a, b| page_updates_cmp(a, b).is_ge()))
        .map(|page_update| page_update[page_update.len() / 2])
        .map(u16::from)
        .sum()
}

pub fn part_2(input: Option<&str>) -> u16 {
    let Output {
        before_map,
        page_updates,
    } = parse_input(input);

    let page_updates_cmp = get_page_updates_cmp(&before_map);

    page_updates
        .into_iter()
        .filter(|page_update| !page_update.is_sorted_by(|a, b| page_updates_cmp(a, b).is_ge()))
        .map(|mut page_update| {
            page_update.sort_by(&page_updates_cmp);
            page_update[page_update.len() / 2]
        })
        .map(u16::from)
        .sum()
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE)), 143);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(Some(EXAMPLE)), 123);
    }
}
