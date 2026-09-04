/// LeetCode #294 - Median of Unsorted Arrays (quickselect)
fn find_median(nums: &mut [i32]) -> f64 {
    let n = nums.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 1 {
        kth(nums, n / 2) as f64
    } else {
        let a = kth(nums, n / 2 - 1) as f64;
        let b = kth(nums, n / 2) as f64;
        (a + b) / 2.0
    }
}

fn kth(nums: &mut [i32], k: usize) -> i32 {
    let mut lo = 0usize;
    let mut hi = nums.len() - 1;
    loop {
        if lo == hi {
            return nums[lo];
        }
        let p = partition(nums, lo, hi);
        if k == p {
            return nums[k];
        } else if k < p {
            hi = p - 1;
        } else {
            lo = p + 1;
        }
    }
}

fn partition(nums: &mut [i32], lo: usize, hi: usize) -> usize {
    let mid = lo + (hi - lo) / 2;
    nums.swap(mid, hi);
    let pivot = nums[hi];
    let mut i = lo;
    for j in lo..hi {
        if nums[j] <= pivot {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, hi);
    i
}

fn main() {
    println!("{}", find_median(&mut [1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::find_median;

    #[test]
    fn example_odd() {
        assert_eq!(find_median(&mut [1, 2, 3]), 2.0);
    }

    #[test]
    fn example_even() {
        assert_eq!(find_median(&mut [1, 2, 3, 4]), 2.5);
    }

    #[test]
    fn unsorted() {
        assert_eq!(find_median(&mut [3, 1, 2]), 2.0);
        assert_eq!(find_median(&mut [4, 1, 3, 2]), 2.5);
    }
}
