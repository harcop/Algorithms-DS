/// LeetCode #3824 - Minimum K to Reduce Array Within Limit
fn minimum_k(nums: Vec<i32>) -> i32 {
    let check = |k: i32| -> bool {
        let mut t = 0i64;
        for &x in &nums {
            t += ((x + k - 1) / k) as i64;
        }
        t <= k as i64 * k as i64
    };
    let mut l = 1;
    let mut r = 100_000;
    while l < r {
        let mid = (l + r) / 2;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}

fn main() {
    println!("{}", minimum_k(vec![3, 7, 5]));
}

#[cfg(test)]
mod tests {
    use super::minimum_k;

    #[test]
    fn example1() {
        assert_eq!(minimum_k(vec![3, 7, 5]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_k(vec![1]), 1);
    }
}
