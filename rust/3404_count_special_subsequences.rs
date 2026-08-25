/// LeetCode #3404 - Count Special Subsequences
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn number_of_subsequences(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;
    let n = nums.len();
    let mut cnt: HashMap<(i32, i32), i64> = HashMap::new();
    for r in 4..n.saturating_sub(2) {
        let c = nums[r];
        for s in r + 2..n {
            let d = nums[s];
            let g = gcd(c, d);
            *cnt.entry((d / g, c / g)).or_insert(0) += 1;
        }
    }
    let mut ans = 0i64;
    for q in 2..n.saturating_sub(4) {
        let b = nums[q];
        for p in 0..(q - 1) {
            let a = nums[p];
            let g = gcd(a, b);
            ans += *cnt.get(&(a / g, b / g)).unwrap_or(&0);
        }
        let c = nums[q + 2];
        for s in q + 4..n {
            let d = nums[s];
            let g = gcd(c, d);
            if let Some(v) = cnt.get_mut(&(d / g, c / g)) {
                *v -= 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", number_of_subsequences(vec![1, 2, 3, 4, 3, 6, 1]));
}

#[cfg(test)]
mod tests {
    use super::number_of_subsequences;

    #[test]
    fn example1() {
        assert_eq!(number_of_subsequences(vec![1, 2, 3, 4, 3, 6, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_subsequences(vec![3, 4, 3, 4, 3, 4, 3, 4]), 3);
    }
}
