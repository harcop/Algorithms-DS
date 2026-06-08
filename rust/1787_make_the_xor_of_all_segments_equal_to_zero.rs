/// LeetCode #1787 - Make the XOR of All Segments Equal to Zero
use std::collections::HashMap;

fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mask = 1 << 10;
    let mut cnt: Vec<HashMap<i32, i32>> = vec![HashMap::new(); k];
    let mut size = vec![0i32; k];
    for (i, &v) in nums.iter().enumerate() {
        *cnt[i % k].entry(v).or_insert(0) += 1;
        size[i % k] += 1;
    }

    let inf = i32::MAX / 4;
    let mut f = vec![inf; mask];
    f[0] = 0;

    for i in 0..k {
        let base = f.iter().copied().min().unwrap() + size[i];
        let mut g = vec![base; mask];
        for j in 0..mask {
            for (&v, &c) in &cnt[i] {
                let prev = f[j ^ v as usize];
                if prev < inf {
                    g[j] = g[j].min(prev + size[i] - c);
                }
            }
        }
        f = g;
    }

    f[0]
}

fn main() {
    println!("{}", min_changes(vec![1, 2, 0, 3, 0], 1));
}

#[cfg(test)]
mod tests {
    use super::min_changes;

    #[test]
    fn example_one() {
        assert_eq!(min_changes(vec![1, 2, 0, 3, 0], 1), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_changes(vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_changes(vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3);
    }
}
