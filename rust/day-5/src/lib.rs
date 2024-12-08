use std::collections::{HashMap, HashSet};

use topo_sort::{SortResults, TopoSort};

/// Find the middle page number of all correctly ordered print requests and sum them
#[allow(dead_code)]
fn day_5_part_1(input: &str) -> i32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let [dependencies, print_jobs, ..] = parts.as_slice() else { unimplemented!() };

    let mut grouped_dependencies: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in dependencies.lines() {
        let nums: Vec<&str> = line.split('|').collect();
        grouped_dependencies
            .entry(nums[0])
            .or_insert_with(HashSet::new)
            .insert(nums[1]);
        grouped_dependencies
            .entry(nums[1])
            .or_insert_with(HashSet::new);
    }

    print_jobs.lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .filter(|line| {
            let mut seen: HashSet<&str> = HashSet::new();
            line.iter().all(|item| {
                let must_be_before = grouped_dependencies.get(*item).expect("item {} was not found in grouped_dependencies");

                if must_be_before.intersection(&seen).count() != 0 {
                    return false;
                }

                seen.insert(item);
                true
            })
        })
        .map(|print_job| {
            let middle_item = print_job[print_job.len() / 2];
            middle_item.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", middle_item))
        })
        .sum()
}

/// Find the middle page number of all incorrectly ordered print requests and sum them
#[allow(dead_code)]
fn day_5_part_2(input: &str) -> i32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let [dependencies, print_jobs, ..] = parts.as_slice() else { unimplemented!() };

    let mut grouped_dependencies: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in dependencies.lines() {
        let nums: Vec<&str> = line.split('|').collect();
        grouped_dependencies
            .entry(nums[0])
            .or_insert_with(HashSet::new)
            .insert(nums[1]);
        grouped_dependencies
            .entry(nums[1])
            .or_insert_with(HashSet::new);
    }

    print_jobs.lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .filter(|line| {
            let mut seen: HashSet<&str> = HashSet::new();
            line.iter().any(|item| {
                let must_be_before = grouped_dependencies.get(*item).expect("item {} was not found in grouped_dependencies");

                if must_be_before.intersection(&seen).count() != 0 {
                    return true;
                }

                seen.insert(item);
                false
            })
        })
        .map(|line| {
            let mut ts = TopoSort::new();
            for item in line {
                let must_be_before = grouped_dependencies.get(item).expect("item {} was not found in grouped_dependencies");
                ts.insert_from_set(item, must_be_before.clone());
            }

            let SortResults::Full(print_job) = ts.into_vec_nodes() else { unimplemented!() };
            let middle_item = print_job[print_job.len() / 2];
            middle_item.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", middle_item))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{day_5_part_1, day_5_part_2};

    #[test]
    fn day_5_part_1_examples() {
        assert_eq!(day_5_part_1(include_str!("example")), 143);
    }

    #[test]
    fn day_5_part_1_test_input() {
        assert_eq!(day_5_part_1(include_str!("input")), 6_034);
    }

    #[test]
    fn day_5_part_2_examples() {
        assert_eq!(day_5_part_2(include_str!("example")), 123);
    }

    #[test]
    fn day_5_part_2_test_input() {
        assert_eq!(day_5_part_2(include_str!("input")), 6_305);
    }
}
