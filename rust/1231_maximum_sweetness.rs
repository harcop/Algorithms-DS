/// LeetCode #1231 - Maximum Sweetness
fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
    let mut lo = 1i32;
    let mut hi: i32 = sweetness.iter().sum();
    let k = k as usize;
    let can = |m: i32| -> bool {
        let mut pieces = 0usize;
        let mut cur = 0i32;
        for &x in &sweetness {
            cur += x;
            if cur >= m {
                pieces += 1;
                cur = 0;
            }
        }
        pieces >= k + 1
    };
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if can(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

fn main() {
    println!("{}", maximize_sweetness(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 5));
}

#[cfg(test)]
mod tests {
    use super::maximize_sweetness;

    #[test]
    fn example_one() {
        assert_eq!(maximize_sweetness(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 5), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximize_sweetness(vec![5, 6, 7, 8, 9, 1, 2, 3, 4], 8), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximize_sweetness(vec![1, 2, 2, 1, 2, 2, 1, 2, 2], 2), 5);
    }
}
