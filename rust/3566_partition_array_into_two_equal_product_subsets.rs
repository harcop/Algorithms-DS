/// LeetCode #3566 - Partition Array into Two Equal Product Subsets
fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
    let n = nums.len();
    for i in 0..(1usize << n) {
        let mut x = 1i64;
        let mut y = 1i64;
        let mut ok = true;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                x *= nums[j] as i64;
            } else {
                y *= nums[j] as i64;
            }
            if x > target || y > target {
                ok = false;
                break;
            }
        }
        if ok && x == target && y == target {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", check_equal_partitions(vec![3, 1, 6, 8, 4], 24));
}

#[cfg(test)]
mod tests {
    use super::check_equal_partitions;

    #[test]
    fn example1() {
        assert_eq!(check_equal_partitions(vec![3, 1, 6, 8, 4], 24), true);
    }

    #[test]
    fn example2() {
        assert_eq!(check_equal_partitions(vec![2, 5, 3, 7], 15), false);
    }
}
