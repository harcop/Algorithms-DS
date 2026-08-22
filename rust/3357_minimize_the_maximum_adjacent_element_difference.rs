/// LeetCode #3357 - Minimize the Maximum Adjacent Element Difference
fn min_difference(nums: Vec<i32>) -> i32 {
    let mut max_positive_gap = 0;
    let mut mn = 1_000_000_000;
    let mut mx = 0;
    for w in nums.windows(2) {
        let (a, b) = (w[0], w[1]);
        if (a == -1) != (b == -1) {
            let positive = a.max(b);
            mn = mn.min(positive);
            mx = mx.max(positive);
        } else {
            max_positive_gap = max_positive_gap.max((a - b).abs());
        }
    }
    let mut l = max_positive_gap;
    let mut r = (mx - mn + 1) / 2;
    while l < r {
        let m = l + (r - l) / 2;
        if check(&nums, m, mn + m, mx - m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

fn check(nums: &[i32], m: i32, x: i32, y: i32) -> bool {
    let mut gap_length = 0;
    let mut prev = 0;
    for &num in nums {
        if num == -1 {
            gap_length += 1;
            continue;
        }
        if prev > 0 && gap_length > 0 {
            if gap_length == 1 && !check_single_gap(prev, num, m, x, y) {
                return false;
            }
            if gap_length > 1 && !check_multiple_gaps(prev, num, m, x, y) {
                return false;
            }
        }
        prev = num;
        gap_length = 0;
    }
    if nums[0] == -1 {
        if let Some(&num) = nums.iter().find(|&&v| v != -1) {
            if !check_boundary_gaps(num, m, x, y) {
                return false;
            }
        }
    }
    if *nums.last().unwrap() == -1 {
        if let Some(&num) = nums.iter().rev().find(|&&v| v != -1) {
            if !check_boundary_gaps(num, m, x, y) {
                return false;
            }
        }
    }
    true
}

fn check_single_gap(a: i32, b: i32, m: i32, x: i32, y: i32) -> bool {
    let gap_x = (a - x).abs().max((b - x).abs());
    let gap_y = (a - y).abs().max((b - y).abs());
    gap_x.min(gap_y) <= m
}

fn check_multiple_gaps(a: i32, b: i32, m: i32, x: i32, y: i32) -> bool {
    let ax = (a - x).abs();
    let ay = (a - y).abs();
    let bx = (b - x).abs();
    let by = (b - y).abs();
    let xy = (x - y).abs();
    let gap_all_x = ax.max(bx);
    let gap_all_y = ay.max(by);
    let gap_x_to_y = ax.max(xy).max(by);
    let gap_y_to_x = ay.max(xy).max(bx);
    gap_all_x.min(gap_all_y).min(gap_x_to_y).min(gap_y_to_x) <= m
}

fn check_boundary_gaps(a: i32, m: i32, x: i32, y: i32) -> bool {
    (a - x).abs().min((a - y).abs()) <= m
}

fn main() {
    println!("{}", min_difference(vec![1, 2, -1, 10, 8]));
}

#[cfg(test)]
mod tests {
    use super::min_difference;

    #[test]
    fn example1() {
        assert_eq!(min_difference(vec![1, 2, -1, 10, 8]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(min_difference(vec![-1, -1, -1]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_difference(vec![-1, 10, -1, 8]), 1);
    }
}
