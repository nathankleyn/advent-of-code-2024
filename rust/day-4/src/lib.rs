/// Find the word "XMAS" forwards and backwards, horizontal, vertical and diagonal
#[allow(dead_code)]
fn day_4_part_1(input: &str) -> i32 {
    // Process the input into a grid
    let grid_horizontal: Vec<String> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_owned())
        .collect();

    // Generate grids for different orientations
    let grid_vertical = rotate_vector_left(&grid_horizontal);
    let grid_diagonal_left = extract_diagonals(&grid_horizontal);
    let grid_diagonal_right = extract_diagonals(&grid_horizontal.iter().rev().cloned().collect());

    count_matches(&grid_horizontal)
        + count_matches(&grid_vertical)
        + count_matches(&grid_diagonal_left)
        + count_matches(&grid_diagonal_right)
}

fn count_matches(vec: &Vec<String>) -> i32 {
    vec.iter()
        .map(|line| {
            line.matches("XMAS").count() as i32 + line.matches("SAMX").count() as i32
        })
        .sum()
}

fn rotate_vector_left(vec: &Vec<String>) -> Vec<String> {
    let mut result = vec![String::new(); vec[0].len()];

    for row in vec {
        for (col_idx, char) in row.chars().enumerate() {
            result[col_idx].push(char);
        }
    }

    result
}

fn extract_diagonals(vec: &Vec<String>) -> Vec<String> {
    let mut diagonals = Vec::new();
    let rows = vec.len();
    let cols = vec[0].len();

    // Extract top-left to bottom-right diagonals
    for d in 0..(rows + cols - 1) {
        let mut diagonal = String::new();
        for row in 0..rows {
            let col = d as isize - row as isize;
            if col >= 0 && col < cols as isize {
                diagonal.push(vec[row].chars().nth(col as usize).unwrap());
            }
        }
        if !diagonal.is_empty() {
            diagonals.push(diagonal);
        }
    }

    diagonals
}

/// Find the word "MAS" or "SAM" in an X formation
#[allow(dead_code)]
fn day_4_part_2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;

    for row in 0..grid.len() - 2 {
        for col in 0..grid[0].len() - 2 {
            if is_x_mas(&grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn is_x_mas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // Extract positions for the "X-MAS" pattern
    let top_left = grid[row][col];
    let top_right = grid[row][col + 2];
    let center = grid[row + 1][col + 1];
    let bottom_left = grid[row + 2][col];
    let bottom_right = grid[row + 2][col + 2];

    // Define the four valid combinations
    let combinations = [
        // Both forward "MAS"
        (top_left == 'M' && top_right == 'S' && center == 'A' && bottom_left == 'M' && bottom_right == 'S'),
        // Both reversed "SAM"
        (top_left == 'S' && top_right == 'M' && center == 'A' && bottom_left == 'S' && bottom_right == 'M'),
        // Top-left and bottom-right forward "MAS", top-right and bottom-left reversed "SAM"
        (top_left == 'M' && top_right == 'M' && center == 'A' && bottom_left == 'S' && bottom_right == 'S'),
        // Top-left and bottom-right reversed "SAM", top-right and bottom-left forward "MAS"
        (top_left == 'S' && top_right == 'S' && center == 'A' && bottom_left == 'M' && bottom_right == 'M'),
    ];

    // Check if any of the combinations match
    combinations.iter().any(|&valid| valid)
}


#[cfg(test)]
mod tests {
    use super::{day_4_part_1, day_4_part_2};

    #[test]
    fn day_4_part_1_examples() {
        assert_eq!(day_4_part_1("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX"), 18);
    }

    #[test]
    fn day_4_part_1_test_input() {
        assert_eq!(day_4_part_1(include_str!("input")), 2_414);
    }

    #[test]
    fn day_4_part_2_examples() {
        assert_eq!(day_4_part_2("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX"), 9);
    }

    #[test]
    fn day_4_part_2_test_input() {
        assert_eq!(day_4_part_2(include_str!("input")), 1_871);
    }
}
