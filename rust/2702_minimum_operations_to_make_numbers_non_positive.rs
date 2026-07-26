/// LeetCode #2702 - Minimum Operations to Make Numbers Non-positive
fn min_operations(nums: Vec<i32>, x: i32, y: i32) -> i32 {
    let check = |t: i32| -> bool {
        let mut cnt: i64 = 0;
        for &v in &nums {
            let thresh = t as i64 * y as i64;
            if v as i64 > thresh {
                cnt += (v as i64 - thresh + (x - y) as i64 - 1) / (x - y) as i64;
            }
        }
        cnt <= t as i64
    };

    let mut l = 0;
    let mut r = *nums.iter().max().unwrap_or(&0);
    while l < r {
        let mid = (l + r) >> 1;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}

fn main() {
    println!("{}", min_operations(vec![3, 4, 1, 7, 6], 4, 2));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(vec![3, 4, 1, 7, 6], 4, 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![1, 2, 1], 2, 1), 1);
    }
}
