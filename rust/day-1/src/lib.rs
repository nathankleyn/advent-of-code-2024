/// Given an input string which is a newline delimited list of 2 numbers per line, split into two
/// lists, sort, and sum the differences between each line.
#[allow(dead_code)]
fn day_1_part_1(input: &str) -> i32 {
    let pairs = day_1_parse(input);
    let mut left: Vec<i32> = pairs.iter().map(|(a, _)| a.clone()).collect();
    let mut right: Vec<i32> = pairs.iter().map(|(_, b)| b.clone()).collect();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(a, b)| {
        (a - b).abs()
    }).sum()
}

/// Given an input string which is a newline delimited list of 2 numbers per line, split into two
/// lists, then multiple every item on the left side with the instances of that number found in the
/// right list.
#[allow(dead_code)]
fn day_1_part_2(input: &str) -> i32 {
    let pairs = day_1_parse(input);
    let mut left: Vec<i32> = pairs.iter().map(|(a, _)| a.clone()).collect();
    let mut right: Vec<i32> = pairs.iter().map(|(_, b)| b.clone()).collect();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().map(|a| {
        let appearances_in_right = right.iter().filter(|b| &a == b).count() as i32;
        a * appearances_in_right
    }).sum()
}

fn day_1_parse(input: &str) -> Vec<(i32, i32)> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let items: Vec<i32> = line.split("   ").map(|item| {
                item.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", item))
            }).collect();

            // Not particularly safe but fine for an AoC
            (items[0], items[1])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{day_1_part_1, day_1_part_2};

    #[test]
    fn day_1_part_1_examples() {
        assert_eq!(day_1_part_1("3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n"), 11);
    }

    #[test]
    fn day_1_part_1_test_input() {
        assert_eq!(day_1_part_1(include_str!("input")), 1_388_114);
    }

    #[test]
    fn day_1_part_2_examples() {
        assert_eq!(day_1_part_2("3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n"), 31);
    }

    #[test]
    fn day_1_part_2_test_input() {
        assert_eq!(day_1_part_2(include_str!("input")), 23_529_853);
    }
}
