/// LeetCode #3960 - Frequency Balance Subarray
use std::collections::HashMap;

fn get_length(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 1usize;
    for l in 0..n {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for r in l..n {
            let x = nums[r];
            let c = cnt.get(&x).copied().unwrap_or(0);
            if c > 0 {
                let f = freq.get(&c).copied().unwrap_or(0) - 1;
                if f == 0 {
                    freq.remove(&c);
                } else {
                    freq.insert(c, f);
                }
            }
            cnt.insert(x, c + 1);
            *freq.entry(c + 1).or_insert(0) += 1;
            let cx = c + 1;
            let balanced = cnt.len() == 1
                || (freq.len() == 2
                    && (freq.get(&(cx * 2)).copied().unwrap_or(0) > 0
                        || (cx % 2 == 0 && freq.get(&(cx / 2)).copied().unwrap_or(0) > 0)));
            if balanced {
                ans = ans.max(r - l + 1);
            }
        }
    }
    ans as i32
}

fn main() {
    println!("{}", get_length(vec![1, 2, 2, 1, 2, 3, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::get_length;

    #[test]
    fn example1() {
        assert_eq!(get_length(vec![1, 2, 2, 1, 2, 3, 3, 3]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(get_length(vec![5, 5, 5, 5]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(get_length(vec![1, 2, 3, 4]), 1);
    }
}
