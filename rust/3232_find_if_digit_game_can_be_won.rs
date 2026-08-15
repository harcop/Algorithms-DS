/// LeetCode #3232 - Find if Digit Game Can Be Won
fn can_alice_win(nums: Vec<i32>) -> bool {
    let a: i32 = nums.iter().filter(|&&x| x < 10).sum();
    let b: i32 = nums.iter().filter(|&&x| x > 9).sum();
    a != b
}

fn main() {
    println!("{}", can_alice_win(vec![1, 2, 3, 4, 10]));
}

#[cfg(test)]
mod tests {
    use super::can_alice_win;

    #[test]
    fn example1() {
        assert!(!can_alice_win(vec![1, 2, 3, 4, 10]));
    }

    #[test]
    fn example2() {
        assert!(can_alice_win(vec![1, 2, 3, 4, 5, 14]));
    }

    #[test]
    fn example3() {
        assert!(can_alice_win(vec![5, 5, 5, 25]));
    }
}
