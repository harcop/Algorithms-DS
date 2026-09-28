/// LeetCode #3996 - Even Number of Knight Moves
fn can_reach(start: Vec<i32>, target: Vec<i32>) -> bool {
    (start[0] + start[1]) % 2 == (target[0] + target[1]) % 2
}

fn main() {
    println!("{}", can_reach(vec![1, 1], vec![2, 2]));
}

#[cfg(test)]
mod tests {
    use super::can_reach;

    #[test]
    fn example1() {
        assert!(can_reach(vec![1, 1], vec![2, 2]));
    }

    #[test]
    fn example2() {
        assert!(!can_reach(vec![4, 5], vec![6, 6]));
    }

    #[test]
    fn same_square() {
        assert!(can_reach(vec![0, 0], vec![0, 0]));
    }
}
