/// LeetCode #3942 - Minimum Operations to Sort a Permutation
fn min_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let zero = nums.iter().position(|&x| x == 0).unwrap();
    let check = |step: i32| -> bool {
        for i in 1..n {
            let prev = (zero as i32 + (i as i32 - 1) * step).rem_euclid(n as i32) as usize;
            let curr = (zero as i32 + i as i32 * step).rem_euclid(n as i32) as usize;
            if nums[prev] > nums[curr] {
                return false;
            }
        }
        true
    };
    let mut ans = i32::MAX;
    if check(1) {
        ans = ans.min(zero as i32);
        ans = ans.min(n as i32 - zero as i32 + 2);
    }
    if check(-1) {
        ans = ans.min(zero as i32 + 2);
        ans = ans.min(n as i32 - zero as i32);
    }
    if ans == i32::MAX { -1 } else { ans }
}

fn main() {
    println!("{}", min_operations(vec![0, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![0, 2, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![1, 0, 2]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![2, 0, 1, 3]), -1);
    }
}
