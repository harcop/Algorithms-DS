/// LeetCode #3388 - Count Beautiful Splits in an Array
fn beautiful_splits(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut lcp = vec![vec![0; n + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (i..n).rev() {
            if nums[i] == nums[j] {
                lcp[i][j] = lcp[i + 1][j + 1] + 1;
            }
        }
    }
    let mut ans = 0;
    for i in 1..n - 1 {
        for j in i + 1..n {
            let a = i <= j - i && lcp[0][i] >= i;
            let b = j - i <= n - j && lcp[i][j] >= j - i;
            if a || b {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", beautiful_splits(vec![1, 1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::beautiful_splits;

    #[test]
    fn example1() {
        assert_eq!(beautiful_splits(vec![1, 1, 2, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(beautiful_splits(vec![1, 2, 3, 4]), 0);
    }
}
