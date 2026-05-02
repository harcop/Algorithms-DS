/// LeetCode #164 - Maximum Gap (linear bucket sort)
fn maximum_gap(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 2 {
        return 0;
    }
    let (mut min_v, mut max_v) = (nums[0], nums[0]);
    for &x in nums.iter().skip(1) {
        min_v = min_v.min(x);
        max_v = max_v.max(x);
    }
    if min_v == max_v {
        return 0;
    }
    let m = n - 1;
    let mut bucket_min = vec![i32::MAX; m];
    let mut bucket_max = vec![i32::MIN; m];
    for x in nums {
        if x == min_v || x == max_v {
            continue;
        }
        let idx = ((x as i64 - min_v as i64) * m as i64 / (max_v as i64 - min_v as i64)) as usize;
        let idx = idx.min(m - 1);
        bucket_min[idx] = bucket_min[idx].min(x);
        bucket_max[idx] = bucket_max[idx].max(x);
    }
    let mut best = 0;
    let mut prev = min_v;
    for i in 0..m {
        if bucket_min[i] == i32::MAX {
            continue;
        }
        best = best.max(bucket_min[i] - prev);
        prev = bucket_max[i];
    }
    best.max(max_v - prev)
}

fn main() {
    println!("{}", maximum_gap(vec![3, 6, 9, 1]));
}

#[cfg(test)]
mod tests {
    use super::maximum_gap;

    #[test]
    fn example_one() {
        assert_eq!(maximum_gap(vec![3, 6, 9, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_gap(vec![10]), 0);
    }
}
