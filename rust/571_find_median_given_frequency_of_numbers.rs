/// LeetCode #571 - Find Median Given Frequency of Numbers
fn find_median(nums: Vec<(i32, i32)>) -> f64 {
    let mut nums = nums;
    nums.sort_by_key(|&(num, _)| num);
    let total: i64 = nums.iter().map(|&(_, f)| f as i64).sum();
    let mut acc = 0i64;
    if total % 2 == 1 {
        let mid = total / 2 + 1;
        for (num, freq) in nums {
            acc += freq as i64;
            if acc >= mid {
                return num as f64;
            }
        }
    } else {
        let m1 = total / 2;
        let m2 = m1 + 1;
        let mut v1 = 0i32;
        let mut v2 = 0i32;
        for (num, freq) in nums {
            acc += freq as i64;
            if acc >= m1 && v1 == 0 && m1 > acc - freq as i64 {
                v1 = num;
            }
            if acc >= m1 && v1 == 0 {
                v1 = num;
            }
            if acc >= m2 {
                v2 = num;
                break;
            }
        }
        return (v1 as f64 + v2 as f64) / 2.0;
    }
    0.0
}

fn main() {
    let nums = vec![(0, 7), (1, 1), (2, 3), (3, 1)];
    println!("{}", find_median(nums));
}

#[cfg(test)]
mod tests {
    use super::find_median;

    #[test]
    fn example() {
        let nums = vec![(0, 7), (1, 1), (2, 3), (3, 1)];
        assert!((find_median(nums) - 0.0).abs() < 1e-9);
    }
}
