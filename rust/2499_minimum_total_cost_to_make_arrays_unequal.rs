/// LeetCode #2499 - Minimum Total Cost to Make Arrays Unequal
fn minimum_total_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let n = nums1.len();
    let mut ans = 0i64;
    let mut same = 0;
    let mut cnt = vec![0; n + 1];

    for i in 0..n {
        if nums1[i] == nums2[i] {
            ans += i as i64;
            same += 1;
            cnt[nums1[i] as usize] += 1;
        }
    }

    let mut m = 0;
    let mut lead = 0;
    for (i, &v) in cnt.iter().enumerate() {
        let t = v * 2 - same;
        if t > 0 {
            m = t;
            lead = i as i32;
            break;
        }
    }

    for i in 0..n {
        if m > 0 && nums1[i] != nums2[i] && nums1[i] != lead && nums2[i] != lead {
            ans += i as i64;
            m -= 1;
        }
    }

    if m > 0 {
        -1
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        minimum_total_cost(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_total_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_total_cost(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            10
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_total_cost(vec![2, 2, 2, 1, 3], vec![1, 2, 2, 3, 3]),
            10
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_total_cost(vec![1, 2, 2], vec![1, 2, 2]), -1);
    }
}
