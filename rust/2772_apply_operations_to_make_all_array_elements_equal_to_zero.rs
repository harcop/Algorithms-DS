/// LeetCode #2772 - Apply Operations to Make All Array Elements Equal to Zero
fn check_array(mut nums: Vec<i32>, k: usize) -> bool {
    let n = nums.len();
    let mut d = vec![0i32; n + 1];
    let mut s = 0i32;
    for i in 0..n {
        s += d[i];
        nums[i] += s;
        if nums[i] == 0 {
            continue;
        }
        if nums[i] < 0 || i + k > n {
            return false;
        }
        s -= nums[i];
        d[i + k] += nums[i];
    }
    true
}

fn main() {
    println!("{}", check_array(vec![2, 2, 3, 1, 1, 0], 3));
}

#[cfg(test)]
mod tests {
    use super::check_array;

    #[test]
    fn example_one() {
        assert!(check_array(vec![2, 2, 3, 1, 1, 0], 3));
    }

    #[test]
    fn example_two() {
        assert!(!check_array(vec![1, 3, 1, 1], 2));
    }
}
