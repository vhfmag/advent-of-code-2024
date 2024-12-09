use itertools::Itertools;

#[derive(Debug)]
struct File {
    id: usize,
    size: u8,
}

#[derive(Debug)]
enum DiskEntry {
    File(File),
    FreeSpace(u8),
}

type DiskMap = Vec<DiskEntry>;

fn parse_input(input: Option<&str>) -> DiskMap {
    let input = if let Some(input) = input {
        input
    } else {
        include_str!("./day9.input")
    }
    .trim();

    let mut file_count = 0;
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let size = c.to_digit(10).expect("Invalid input: couldn't parse size") as u8;
            if i % 2 == 0 {
                let id = file_count;
                file_count += 1;
                DiskEntry::File(File { id, size })
            } else {
                DiskEntry::FreeSpace(size)
            }
        })
        .collect()
}

type NumericalRepr = Vec<Option<usize>>;

fn get_numerical_representation(disk: &Vec<DiskEntry>) -> NumericalRepr {
    disk.into_iter()
        .flat_map(|entry| match entry {
            DiskEntry::File(File { id, size }) => {
                std::iter::repeat_n(Some(*id), usize::from(*size))
            }
            DiskEntry::FreeSpace(size) => std::iter::repeat_n(None, usize::from(*size)),
        })
        .collect()
}

#[allow(dead_code)]
fn numerical_representation_to_string(disk: &NumericalRepr) -> String {
    disk.iter()
        .map(|el| el.map(|n| n.to_string()).unwrap_or(".".to_string()))
        .join("")
}

pub fn part_1(input: Option<&str>) -> usize {
    let map = parse_input(input);

    let mut disk = get_numerical_representation(&map);

    let mut compacted_disk = vec![];

    while disk.len() > 0 {
        let el = disk.remove(0);
        if let Some(el) = el {
            compacted_disk.push(el);
        } else {
            while disk.last() == Some(&None) {
                disk.pop();
            }
            compacted_disk.push(disk.pop().expect("Invalid input: couldn't pop").expect(
                "Unreachable: element is None even though we removed all trailing 'None's just now",
            ));
        }
    }

    compacted_disk
        .iter()
        .enumerate()
        .map(|(i, el)| el * i)
        .sum()
}

pub fn part_2(input: Option<&str>) -> usize {
    let mut disk = parse_input(input);

    let &max_id = disk
        .iter()
        .rev()
        .find_map(|entry| match entry {
            DiskEntry::File(File { id, .. }) => Some(id),
            _ => None,
        })
        .expect("Invalid input: no files found");

    for id in (0..=max_id).rev() {
        let (file_idx, file_size) = disk
            .iter()
            .enumerate()
            .find_map(|(idx, entry)| match entry {
                DiskEntry::File(file) if id == file.id => Some((idx, file.size)),
                _ => None,
            })
            .ok_or_else(|| format!("Invalid input: missing file with id {id}"))
            .unwrap();

        let first_matching_space = disk
            .iter()
            .enumerate()
            .find_map(|(idx, entry)| match entry {
                DiskEntry::FreeSpace(size) if *size >= file_size && idx < file_idx => {
                    Some((idx, size))
                }
                _ => None,
            });

        if let Some((space_idx, &space_size)) = first_matching_space {
            use std::iter::once;

            let entry = std::mem::replace(&mut disk[file_idx], DiskEntry::FreeSpace(file_size));

            if file_size < space_size {
                disk.splice(
                    space_idx..=space_idx,
                    once(entry).chain(once(DiskEntry::FreeSpace(space_size - file_size))),
                );
            } else {
                disk.splice(space_idx..=space_idx, once(entry));
            }
        }
    }

    let num_repr = get_numerical_representation(&disk);

    num_repr
        .iter()
        .enumerate()
        .map(|(i, el)| el.map(|el| el * i).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod test {
    static EXAMPLE: &str = r"
2333133121414131402
";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(Some(EXAMPLE)), 1928);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(Some(EXAMPLE)), 2858);
    }
}
