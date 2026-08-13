/// LeetCode #3176 - Find the Maximum Length of a Good Subsequence I
fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut f = vec![vec![1; k + 1]; n];
    let mut ans = 0;
    for i in 0..n {
        for h in 0..=k {
            for j in 0..i {
                if nums[i] == nums[j] {
                    f[i][h] = f[i][h].max(f[j][h] + 1);
                } else if h > 0 {
                    f[i][h] = f[i][h].max(f[j][h - 1] + 1);
                }
            }
            ans = ans.max(f[i][k]);
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_length(vec![1, 2, 1, 1, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_length;

    #[test]
    fn example1() {
        assert_eq!(maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
    }
}
