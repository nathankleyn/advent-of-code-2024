/// Given an input string which is a newline delimited list of n numbers per line, split into rows
/// and check whether each row satisfies some rules
#[allow(dead_code)]
fn day_2_part_1(input: &str) -> i32 {
    let lines = day_1_parse(input);

    lines.iter().filter(|line| is_safe_line(line)).count() as i32
}

/// Given an input string which is a newline delimited list of n numbers per line, split into rows
/// and check whether each row satisfies some rules
#[allow(dead_code)]
fn day_2_part_2(input: &str) -> i32 {
    let lines = day_1_parse(input);

    lines.iter().filter(|line| {
        if is_safe_line(line) {
            return true;
        }

        for i in 0..(line.len()) {
            let mut skipped_line = line.to_owned().clone();
            skipped_line.remove(i);

            if is_safe_line(&skipped_line) {
                return true;
            }
        }

        return false;
    }).count() as i32
}

fn is_safe_line(line: &Vec<i32>) -> bool {
    let increasing = (line[1] - line[0]).is_positive();
    line.windows(2).all(|xs| {
        let diff = xs[1] - xs[0];
        diff.abs() >= 1 && diff.abs() <= 3 && (increasing ^ diff.is_negative())
    })
}

fn day_1_parse(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(" ").map(|item| {
                item.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", item))
            }).collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{day_2_part_1, day_2_part_2};

    #[test]
    fn day_2_part_1_examples() {
        assert_eq!(day_2_part_1("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"), 2);
    }

    #[test]
    fn day_2_part_1_test_input() {
        assert_eq!(day_2_part_1(include_str!("input")), 421);
    }

    #[test]
    fn day_2_part_2_examples() {
        assert_eq!(day_2_part_2("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"), 4);
    }

    #[test]
    fn day_2_part_2_test_input() {
        assert_eq!(day_2_part_2(include_str!("input")), 476);
    }
}
