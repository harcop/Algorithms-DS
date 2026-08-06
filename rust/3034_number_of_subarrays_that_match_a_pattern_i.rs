/// LeetCode #3034 - Number of Subarrays That Match a Pattern I
fn compare(a: i32, b: i32) -> i32 {
    if a == b {
        0
    } else if a < b {
        1
    } else {
        -1
    }
}

fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
    let m = pattern.len();
    let n = nums.len();
    if n < m + 1 {
        return 0;
    }
    let mut ans = 0;
    for i in 0..=n - m - 1 {
        let mut ok = true;
        for j in 0..m {
            if compare(nums[i + j], nums[i + j + 1]) != pattern[j] {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]));
}

#[cfg(test)]
mod tests {
    use super::count_matching_subarrays;

    #[test]
    fn example1() {
        assert_eq!(
            count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_matching_subarrays(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]),
            2
        );
    }
}
