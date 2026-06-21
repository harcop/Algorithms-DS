/// LeetCode #2011 - Final Value of Variable After Performing Operations
fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations
        .iter()
        .map(|s| if s.as_bytes()[1] == b'+' { 1 } else { -1 })
        .sum()
}

fn main() {
    println!(
        "{}",
        final_value_after_operations(vec!["--X".into(), "X++".into(), "X++".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::final_value_after_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            final_value_after_operations(vec!["--X".into(), "X++".into(), "X++".into()]),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            final_value_after_operations(vec!["++X".into(), "++X".into(), "X++".into()]),
            3
        );
    }
}
