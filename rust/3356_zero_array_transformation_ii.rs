/// LeetCode #3356 - Zero Array Transformation II
fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let m = queries.len();
    let check = |k: usize| -> bool {
        let mut d = vec![0i32; n + 1];
        for q in &queries[..k] {
            d[q[0] as usize] += q[2];
            d[q[1] as usize + 1] -= q[2];
        }
        let mut s = 0;
        for i in 0..n {
            s += d[i];
            if nums[i] > s {
                return false;
            }
        }
        true
    };
    let mut l = 0usize;
    let mut r = m + 1;
    while l < r {
        let mid = (l + r) / 2;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    if l > m {
        -1
    } else {
        l as i32
    }
}

fn main() {
    println!(
        "{}",
        min_zero_array(vec![2, 0, 2], vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_zero_array;

    #[test]
    fn example1() {
        assert_eq!(
            min_zero_array(vec![2, 0, 2], vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]),
            -1
        );
    }
}
