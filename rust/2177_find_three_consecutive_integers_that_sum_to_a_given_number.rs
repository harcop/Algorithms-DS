/// LeetCode #2177 - Find Three Consecutive Integers That Sum to a Given Number
fn sum_of_three(num: i64) -> Vec<i64> {
    if num % 3 != 0 {
        return Vec::new();
    }
    let mid = num / 3;
    vec![mid - 1, mid, mid + 1]
}

fn main() {
    println!("{:?}", sum_of_three(33));
}

#[cfg(test)]
mod tests {
    use super::sum_of_three;

    #[test]
    fn example_one() {
        assert_eq!(sum_of_three(33), vec![10, 11, 12]);
    }

    #[test]
    fn example_two() {
        assert!(sum_of_three(4).is_empty());
    }
}
