/// LeetCode #3134 - Find the Median of the Uniqueness Array
use std::collections::HashMap;

fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = (1 + n) * n / 2;
    let need = (m + 1) / 2;
    let check = |mx: i32| -> bool {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut l = 0usize;
        let mut k = 0usize;
        for r in 0..n {
            *cnt.entry(nums[r]).or_insert(0) += 1;
            while cnt.len() > mx as usize {
                let y = nums[l];
                let e = cnt.get_mut(&y).unwrap();
                *e -= 1;
                if *e == 0 {
                    cnt.remove(&y);
                }
                l += 1;
            }
            k += r + 1 - l;
            if k >= need {
                return true;
            }
        }
        false
    };
    let mut lo = 0;
    let mut hi = n as i32;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if check(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    println!("{}", median_of_uniqueness_array(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::median_of_uniqueness_array;

    #[test]
    fn example1() {
        assert_eq!(median_of_uniqueness_array(vec![1, 2, 3]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(median_of_uniqueness_array(vec![3, 4, 3, 4, 5]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(median_of_uniqueness_array(vec![4, 3, 5, 4]), 2);
    }
}
