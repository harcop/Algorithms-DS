/// LeetCode #3191 - Minimum Operations to Make Binary Array Elements Equal to One I
fn min_operations(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        if nums[i] == 0 {
            if i + 2 >= n {
                return -1;
            }
            nums[i + 1] ^= 1;
            nums[i + 2] ^= 1;
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", min_operations(vec![0, 1, 1, 1, 0, 0]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![0, 1, 1, 1]), -1);
    }
}
