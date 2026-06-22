/// LeetCode #2035 - Partition Array Into Two Arrays to Minimize Sum Difference
use std::collections::HashMap;

fn minimum_difference(nums: Vec<i32>) -> i32 {
    let n = nums.len() / 2;
    let mut f: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();

    for i in 0..(1 << n) {
        let mut s = 0i32;
        let mut cnt = 0i32;
        let mut s1 = 0i32;
        let mut cnt1 = 0i32;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                s += nums[j];
                cnt += 1;
                s1 += nums[n + j];
                cnt1 += 1;
            } else {
                s -= nums[j];
                s1 -= nums[n + j];
            }
        }
        f.entry(cnt).or_default().push(s);
        g.entry(cnt1).or_default().push(s1);
    }

    let mut ans = i32::MAX;
    for i in 0..=n as i32 {
        let Some(fi) = f.get(&i) else { continue };
        let Some(gi) = g.get(&(n as i32 - i)) else { continue };
        let mut fi = fi.clone();
        let mut gi = gi.clone();
        fi.sort_unstable();
        gi.sort_unstable();
        for &a in &fi {
            let b = -a;
            let left = gi.partition_point(|&x| x < b);
            if left < gi.len() {
                ans = ans.min((a + gi[left]).abs());
            }
            if left > 0 {
                ans = ans.min((a + gi[left - 1]).abs());
            }
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_difference(vec![3, 9, 7, 3]));
}

#[cfg(test)]
mod tests {
    use super::minimum_difference;

    #[test]
    fn example_one() {
        assert_eq!(minimum_difference(vec![3, 9, 7, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_difference(vec![-36, 36]), 72);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_difference(vec![2, -1, 0, 4, -2, -9]), 0);
    }
}
