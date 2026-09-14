/// LeetCode #3728 - Stable Subarrays With Equal Boundary and Interior Sum
use std::collections::HashMap;

fn count_stable_subarrays(capacity: Vec<i32>) -> i64 {
    let n = capacity.len();
    let mut s = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + capacity[i] as i64;
    }
    let mut cnt: HashMap<(i32, i64), i64> = HashMap::new();
    let mut ans = 0i64;
    for r in 2..n {
        let l = r - 2;
        *cnt.entry((capacity[l], capacity[l] as i64 + s[l + 1]))
            .or_insert(0) += 1;
        ans += cnt.get(&(capacity[r], s[r])).copied().unwrap_or(0);
    }
    ans
}

fn main() {
    println!("{}", count_stable_subarrays(vec![9, 3, 3, 3, 9]));
}

#[cfg(test)]
mod tests {
    use super::count_stable_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_stable_subarrays(vec![9, 3, 3, 3, 9]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_stable_subarrays(vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(count_stable_subarrays(vec![-4, 4, 0, 0, -8, -4]), 1);
    }
}
