/// LeetCode #324 - Wiggle Sort II (median + virtual index partition)
fn wiggle_sort(nums: &mut Vec<i32>) {
    let n = nums.len();
    if n <= 1 {
        return;
    }
    let median = {
        let (_, m, _) = nums.select_nth_unstable(n / 2);
        *m
    };
    let map = |i: usize| -> usize { (1 + 2 * i) % (n | 1) };
    let mut i = 0usize;
    let mut j = 0usize;
    let mut k = n - 1;
    while j <= k {
        if nums[map(j)] > median {
            nums.swap(map(i), map(j));
            i += 1;
            j += 1;
        } else if nums[map(j)] < median {
            nums.swap(map(j), map(k));
            k -= 1;
        } else {
            j += 1;
        }
    }
}

fn main() {
    let mut v = vec![1, 5, 1, 1, 6, 4];
    wiggle_sort(&mut v);
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::wiggle_sort;

    fn ok_wiggle(nums: &[i32]) -> bool {
        let n = nums.len();
        for i in 0..n {
            if i + 1 < n && i % 2 == 0 && !(nums[i] < nums[i + 1]) {
                return false;
            }
            if i + 1 < n && i % 2 == 1 && !(nums[i] > nums[i + 1]) {
                return false;
            }
        }
        true
    }

    #[test]
    fn example() {
        let mut nums = vec![1, 5, 1, 1, 6, 4];
        let perm = nums.clone();
        wiggle_sort(&mut nums);
        let mut counts = std::collections::HashMap::new();
        for x in &perm {
            *counts.entry(*x).or_insert(0) += 1;
        }
        for x in &nums {
            let c = counts.get_mut(x).unwrap();
            *c -= 1;
        }
        assert!(counts.values().all(|&c| c == 0));
        assert!(ok_wiggle(&nums));
    }
}
