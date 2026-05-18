/// LeetCode #873 - Length of Longest Fibonacci Subsequence
use std::collections::HashMap;

fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    if n < 3 {
        return 0;
    }
    let index: HashMap<i32, usize> = arr.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut dp = vec![vec![2; n]; n];
    let mut best = 0;
    for j in 0..n {
        for i in 0..j {
            let need = arr[j] - arr[i];
            if let Some(&k) = index.get(&need) {
                if k < i {
                    dp[i][j] = dp[k][i] + 1;
                    best = best.max(dp[i][j]);
                }
            }
        }
    }
    if best < 3 {
        0
    } else {
        best
    }
}

fn main() {
    println!("{}", len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]));
}

#[cfg(test)]
mod tests {
    use super::len_longest_fib_subseq;

    #[test]
    fn example_one() {
        assert_eq!(len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]), 5);
    }
}
