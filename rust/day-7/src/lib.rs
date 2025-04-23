/// Find the number of touched squares in a map when navigating past obstructions
#[allow(dead_code)]
fn day_5_part_1(input: &str) -> i32 {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let xs: Vec<&str> = line.split(':').collect();

            let res =  xs[0].parse::<i32>().expect(&format!("Could not parse '{}' as i32.", xs[0]));
            let parts: Vec<i32> = xs[1].trim().split(' ').map(|x| {
                x.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", x))
            }).collect();
        });

    0
}

#[cfg(test)]
mod tests {
    use super::day_5_part_1;

    #[test]
    fn day_6_part_1_examples() {
        assert_eq!(day_5_part_1(include_str!("example")), 3_749);
    }

    #[test]
    fn day_6_part_1_test_input() {
        assert_eq!(day_5_part_1(include_str!("input")), 0);
    }
}
