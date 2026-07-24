/// LeetCode #2638 - Count the Number of K-Free Subsets
use std::collections::HashMap;

fn count_the_num_of_k_free_subsets(mut nums: Vec<i32>, k: i32) -> i64 {
    nums.sort_unstable();
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for x in nums {
        g.entry(x % k).or_default().push(x);
    }
    let mut ans = 1i64;
    for arr in g.values() {
        let m = arr.len();
        let mut f = vec![0i64; m + 1];
        f[0] = 1;
        if m >= 1 {
            f[1] = 2;
        }
        for i in 2..=m {
            if arr[i - 1] - arr[i - 2] == k {
                f[i] = f[i - 1] + f[i - 2];
            } else {
                f[i] = f[i - 1] * 2;
            }
        }
        ans *= f[m];
    }
    ans
}

fn main() {
    println!("{}", count_the_num_of_k_free_subsets(vec![5, 4, 6], 1));
}

#[cfg(test)]
mod tests {
    use super::count_the_num_of_k_free_subsets;

    #[test]
    fn example_one() {
        assert_eq!(count_the_num_of_k_free_subsets(vec![5, 4, 6], 1), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_the_num_of_k_free_subsets(vec![2, 3, 5, 8], 5), 12);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_the_num_of_k_free_subsets(vec![10, 5, 9, 11], 20), 16);
    }
}
