/// LeetCode #1502 - Can Make Arithmetic Progression From Sequence
fn can_make_arithmetic_progression_from_sequence(arr: Vec<i32>) -> bool {
    if arr.len() <= 2 {
        return true;
    }
    let mut a = arr;
    a.sort_unstable();
    let d = a[1] - a[0];
    a.windows(2).all(|w| w[1] - w[0] == d)
}

fn main() {
    println!("{}", can_make_arithmetic_progression_from_sequence(vec![3, 5, 1]));
}

#[cfg(test)]
mod tests {
    use super::can_make_arithmetic_progression_from_sequence;

    #[test]
    fn example_one() {
        assert!(can_make_arithmetic_progression_from_sequence(vec![3, 5, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!can_make_arithmetic_progression_from_sequence(vec![1, 2, 4]));
    }
}
