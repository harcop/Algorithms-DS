/// LeetCode #3229 - Minimum Operations to Make Array Equal to Target
fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut f = (target[0] - nums[0]).abs() as i64;
    for i in 1..n {
        let x = target[i] - nums[i];
        let y = target[i - 1] - nums[i - 1];
        if x * y > 0 {
            let d = x.abs() - y.abs();
            if d > 0 {
                f += d as i64;
            }
        } else {
            f += x.abs() as i64;
        }
    }
    f
}

fn main() {
    println!("{}", minimum_operations(vec![3, 5, 1, 2], vec![4, 6, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_operations(vec![3, 5, 1, 2], vec![4, 6, 2, 4]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_operations(vec![1, 3, 2], vec![2, 1, 4]), 5);
    }
}
