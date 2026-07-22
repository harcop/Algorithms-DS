/// LeetCode #2579 - Count Total Number of Colored Cells
fn colored_cells(n: i32) -> i64 {
    let n = n as i64;
    2 * n * (n - 1) + 1
}

fn main() {
    println!("{}", colored_cells(1));
}

#[cfg(test)]
mod tests {
    use super::colored_cells;

    #[test]
    fn example_one() {
        assert_eq!(colored_cells(1), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(colored_cells(2), 5);
    }
}
