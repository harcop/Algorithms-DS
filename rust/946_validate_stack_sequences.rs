/// LeetCode #946 - Validate Stack Sequences

fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = Vec::new();
    let mut j = 0usize;
    for &x in &pushed {
        stack.push(x);
        while j < popped.len() && !stack.is_empty() && *stack.last().unwrap() == popped[j] {
            stack.pop();
            j += 1;
        }
    }
    j == popped.len()
}

fn main() {
    println!("{}", validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::validate_stack_sequences;

    #[test]
    fn example_one() {
        assert!(validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]));
    }
}
