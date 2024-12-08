use std::collections::HashSet;

enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug, PartialEq)]
enum MapEntry {
    Space,
    Obstruction,
}

/// Find the number of touched squares in a map when navigating past obstructions
#[allow(dead_code)]
fn day_5_part_1(input: &str) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let (initial_guard_pos, map) = parse_map(input);
    run_map(&map, initial_guard_pos, &mut visited).expect("Must not have looped")
}

/// Find the number of variants of the map where a single obstruction could be
/// added and cause the guard to enter a loop
#[allow(dead_code)]
fn day_5_part_2(input: &str) -> i32 {
    let (initial_guard_pos, map) = parse_map(input);

    // Run the first untouched map to find the squares we touch — these are the only places we can add obstructions to
    // since any other path we'd never hit anything we add
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    run_map(&map, initial_guard_pos, &mut visited).expect("Must not have looped");


    visited.iter().filter(|visited_pos| {
        let mut mutated_map = map.clone();
        mutated_map[visited_pos.1 as usize][visited_pos.0 as usize] = MapEntry::Obstruction;

        run_map(&mutated_map, initial_guard_pos, &mut HashSet::new()).is_none()
    }).count() as i32
}

fn parse_map(input: &str) -> ((i32, i32), Vec<Vec<MapEntry>>) {
    let mut guard_pos: (i32, i32) = (0, 0);
    let map: Vec<Vec<MapEntry>> = input.lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(|(x, map_entry)| {
                match map_entry {
                    '.' => MapEntry::Space,
                    '#' => MapEntry::Obstruction,
                    '^' => {
                        guard_pos = (x as i32, y as i32);
                        MapEntry::Space
                    },
                    _ => unimplemented!(),
                }
            }).collect()
        })
        .collect();

    (guard_pos, map)
}

fn run_map(
    map: &Vec<Vec<MapEntry>>,
    initial_guard_pos: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
) -> Option<i32> {
    let mut guard_dir: GuardDirection = GuardDirection::Up;
    let mut guard_pos: (i32, i32) = initial_guard_pos;

    visited.insert(initial_guard_pos);

    let mut visited_in_a_row = 0;

    loop {
        let original_pos = guard_pos;

        guard_pos = match guard_dir {
            GuardDirection::Up => (guard_pos.0, guard_pos.1 - 1),
            GuardDirection::Right => (guard_pos.0 + 1, guard_pos.1),
            GuardDirection::Down => (guard_pos.0, guard_pos.1 + 1),
            GuardDirection::Left => (guard_pos.0 - 1, guard_pos.1),
        };

        if guard_pos.0 < 0 || guard_pos.0 >= map.len() as i32 {
            break;
        }

        if guard_pos.1 < 0 || guard_pos.1 >= map[0].len() as i32 {
            break;
        }

        if map[guard_pos.1 as usize][guard_pos.0 as usize] == MapEntry::Obstruction {
            guard_pos = original_pos;
            guard_dir = match guard_dir {
                GuardDirection::Up => GuardDirection::Right,
                GuardDirection::Right => GuardDirection::Down,
                GuardDirection::Down => GuardDirection::Left,
                GuardDirection::Left => GuardDirection::Up,
            };
        } else {
            if visited.contains(&guard_pos) {
                visited_in_a_row += 1;
            } else {
                visited_in_a_row = 0;
            }

            if visited_in_a_row == 10000 {
                return None;
            }

            visited.insert(guard_pos);
        }
    }

    Some(visited.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::{day_5_part_1, day_5_part_2};

    #[test]
    fn day_6_part_1_examples() {
        assert_eq!(day_5_part_1(include_str!("example")), 41);
    }

    #[test]
    fn day_6_part_1_test_input() {
        assert_eq!(day_5_part_1(include_str!("input")), 4_982);
    }

    #[test]
    fn day_6_part_2_examples() {
        assert_eq!(day_5_part_2(include_str!("example")), 6);
    }

    #[test]
    fn day_6_part_2_test_input() {
        assert_eq!(day_5_part_2(include_str!("input")), 1_663);
    }
}
