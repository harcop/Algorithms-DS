/// LeetCode #3795 - Minimum Subarray Length With Distinct Sum At Least K
use std::collections::HashMap;

fn min_length(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut ans = n + 1;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut l = 0;
    let mut s = 0i64;
    let k = k as i64;
    for r in 0..n {
        let x = nums[r];
        let e = cnt.entry(x).or_insert(0);
        *e += 1;
        if *e == 1 {
            s += x as i64;
        }
        while s >= k {
            ans = ans.min(r - l + 1);
            let y = nums[l];
            let e = cnt.get_mut(&y).unwrap();
            *e -= 1;
            if *e == 0 {
                s -= y as i64;
            }
            l += 1;
        }
    }
    if ans > n {
        -1
    } else {
        ans as i32
    }
}

fn main() {
    println!("{}", min_length(vec![2, 2, 3, 1], 4));
}

#[cfg(test)]
mod tests {
    use super::min_length;

    #[test]
    fn example1() {
        assert_eq!(min_length(vec![2, 2, 3, 1], 4), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_length(vec![3, 2, 3, 4], 5), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_length(vec![5, 5, 4], 5), 1);
    }
}
