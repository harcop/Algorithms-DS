/// LeetCode #923 - 3Sum With Multiplicity
const MOD923: i64 = 1_000_000_007;

fn three_sum_multi(mut arr: Vec<i32>, target: i32) -> i32 {
    arr.sort_unstable();
    let n = arr.len();
    let mut ans: i64 = 0;
    for i in 0..n {
        let t = target - arr[i];
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            let sum = arr[j] as i64 + arr[k] as i64;
            if sum < t as i64 {
                j += 1;
            } else if sum > t as i64 {
                k -= 1;
            } else if arr[j] != arr[k] {
                let mut cnt_j = 1usize;
                while j + cnt_j < k && arr[j + cnt_j] == arr[j] {
                    cnt_j += 1;
                }
                let mut cnt_k = 1usize;
                while j + cnt_k <= k && arr[k - cnt_k] == arr[k] {
                    cnt_k += 1;
                }
                ans = (ans + (cnt_j * cnt_k) as i64) % MOD923;
                j += cnt_j;
                k -= cnt_k;
            } else {
                let m = (k - j + 1) as i64;
                ans = (ans + m * (m - 1) / 2) % MOD923;
                break;
            }
        }
    }
    ans as i32
}

fn main() {
    println!("{}", three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8));
}

#[cfg(test)]
mod tests {
    use super::three_sum_multi;

    #[test]
    fn example_one() {
        assert_eq!(three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8), 20);
    }

    #[test]
    fn example_two() {
        assert_eq!(three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5), 12);
    }
}
