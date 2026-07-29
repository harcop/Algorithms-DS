/// LeetCode #2784 - Check if Array is Good
fn is_good(nums: Vec<i32>) -> bool {
    let n = nums.len() - 1;
    let mut cnt = [0i32; 201];
    for &x in &nums {
        cnt[x as usize] += 1;
    }
    if cnt[n] != 2 {
        return false;
    }
    (1..n).all(|i| cnt[i] == 1)
}

fn main() {
    println!("{}", is_good(vec![1, 3, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::is_good;

    #[test]
    fn example_one() {
        assert!(!is_good(vec![2, 1, 3]));
    }

    #[test]
    fn example_two() {
        assert!(is_good(vec![1, 3, 3, 2]));
    }

    #[test]
    fn example_three() {
        assert!(is_good(vec![1, 1]));
    }

    #[test]
    fn example_four() {
        assert!(!is_good(vec![3, 4, 4, 1, 2, 1]));
    }
}
