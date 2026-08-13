/// LeetCode #3192 - Minimum Operations to Make Binary Array Elements Equal to One II
fn min_operations(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut v = 0;
    for x in nums {
        if (x ^ v) == 0 {
            ans += 1;
            v ^= 1;
        }
    }
    ans
}

fn main() {
    println!("{}", min_operations(vec![0, 1, 1, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![0, 1, 1, 0, 1]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![1, 0, 0, 0]), 1);
    }
}
