/// LeetCode #2878 - Get the Size of a DataFrame (Pandas; Rust analogue)
fn get_dataframe_size(rows: usize, columns: usize) -> [usize; 2] {
    [rows, columns]
}

fn main() {
    println!("{:?}", get_dataframe_size(10, 5));
}

#[cfg(test)]
mod tests {
    use super::get_dataframe_size;

    #[test]
    fn example() {
        assert_eq!(get_dataframe_size(10, 5), [10, 5]);
    }
}
