/// LeetCode #2607 - Make K-Subarray Sums Equal
fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }

    let n = arr.len();
    let g = gcd(n, k as usize);
    let mut ans = 0i64;
    for i in 0..g {
        let mut t: Vec<i32> = (i..n).step_by(g).map(|j| arr[j]).collect();
        t.sort_unstable();
        let mid = t[t.len() / 2];
        for x in t {
            ans += (x - mid).abs() as i64;
        }
    }
    ans
}

fn main() {
    println!("{}", make_sub_k_sum_equal(vec![1, 4, 1, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::make_sub_k_sum_equal;

    #[test]
    fn example_one() {
        assert_eq!(make_sub_k_sum_equal(vec![1, 4, 1, 3], 2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(make_sub_k_sum_equal(vec![2, 5, 5, 7], 3), 5);
    }
}
