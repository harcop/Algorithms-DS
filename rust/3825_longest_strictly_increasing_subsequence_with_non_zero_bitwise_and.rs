/// LeetCode #3825 - Longest Strictly Increasing Subsequence With Non-Zero Bitwise AND
fn longest_subsequence(nums: Vec<i32>) -> i32 {
    fn lis(arr: &[i32]) -> i32 {
        let mut g = Vec::new();
        for &x in arr {
            let j = g.partition_point(|&y| y < x);
            if j == g.len() {
                g.push(x);
            } else {
                g[j] = x;
            }
        }
        g.len() as i32
    }
    let mx = *nums.iter().max().unwrap();
    let m = if mx == 0 { 0 } else { 32 - mx.leading_zeros() };
    let mut ans = 0;
    for i in 0..m {
        let arr: Vec<i32> = nums.iter().copied().filter(|&x| (x >> i) & 1 == 1).collect();
        ans = ans.max(lis(&arr));
    }
    ans
}

fn main() {
    println!("{}", longest_subsequence(vec![5, 4, 7]));
}

#[cfg(test)]
mod tests {
    use super::longest_subsequence;

    #[test]
    fn example1() {
        assert_eq!(longest_subsequence(vec![5, 4, 7]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_subsequence(vec![2, 3, 6]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_subsequence(vec![0, 1]), 1);
    }
}
