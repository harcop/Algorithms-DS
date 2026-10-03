/// LeetCode #4062 - Transform Array Using Pair Operations
fn can_transform(source: Vec<i32>, target: Vec<i32>) -> bool {
    let mut diff = 0i64;
    for i in 0..source.len() {
        diff += source[i] as i64 - target[i] as i64;
    }
    diff == 0
}

fn main() {
    println!("{}", can_transform(vec![1, 2, 3], vec![0, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::can_transform;

    #[test]
    fn example1() {
        assert!(can_transform(vec![1, 2, 3], vec![0, 2, 4]));
    }

    #[test]
    fn example2() {
        assert!(can_transform(vec![-5, -5], vec![-15, 5]));
    }

    #[test]
    fn example3() {
        assert!(!can_transform(vec![1, 2, 1], vec![0, 2, 5]));
    }
}
