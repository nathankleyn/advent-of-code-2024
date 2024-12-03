use regex::Regex;

/// Given an input string which is a piece of corrupted memory, find the valid instructions and
/// filter out invalid instructions then execute and return the result
#[allow(dead_code)]
fn day_3_part_1(input: &str) -> i32 {
    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").expect("Could not parse Regex");

    re.captures_iter(input).map(|c| c.extract()).map(|(_, [a, b])| {
        let a = a.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", a));
        let b = b.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", a));

        a * b
    }).sum()
}

/// Given an input string which is a piece of corrupted memory, find the valid instructions and
/// filter out invalid instructions then execute and return the result. Handle the 'don't' and 'do'
/// operators.
#[allow(dead_code)]
fn day_3_part_2(input: &str) -> i32 {
    let re = Regex::new("^mul\\((\\d+),(\\d+)\\)").expect("Could not parse Regex");

    let mut chars: Vec<char> = input.chars().collect();
    let mut instructions_enabled = true;

    // Push 4 spaces to the end to ensure we can read the a valid instruction that could be right at the end of the
    // string. Since the shortest possible instruction is mul(x,y) where x and y are one digit, and this string is 8
    // chars, we only need to add 4 spaces to the end to ensure this will be at the start of the last window.
    for _ in 0..4 {
        chars.push(' ');
    }

    // We can pick a window size of 12 because the documentation says mul takes at most two 3 digit
    // numbers
    chars.windows(12).map(|window| {
        let window_as_str: String = window.iter().collect();

        if window_as_str.starts_with("don't()") {
            instructions_enabled = false;
            return 0;
        }

        if window_as_str.starts_with("do()") {
            instructions_enabled = true;
            return 0;
        }

        if !instructions_enabled {
            return 0;
        }

        if window_as_str.starts_with("mul(") {
            return re.captures_iter(&window_as_str).map(|c| c.extract()).map(|(_, [a, b])| {
                let a = a.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", a));
                let b = b.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", a));

                a * b
            }).sum()
        }

        return 0;
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::{day_3_part_1, day_3_part_2};

    #[test]
    fn day_3_part_1_examples() {
        assert_eq!(day_3_part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }

    #[test]
    fn day_3_part_1_test_input() {
        assert_eq!(day_3_part_1(include_str!("input")), 183_380_722);
    }

    #[test]
    fn day_3_part_2_examples() {
        assert_eq!(day_3_part_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }

    #[test]
    fn day_3_part_2_test_input() {
        assert_eq!(day_3_part_2(include_str!("input")), 82_733_683);
    }
}
