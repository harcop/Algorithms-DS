/// LeetCode #3355 - Zero Array Transformation I
fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
    let n = nums.len();
    let mut d = vec![0i32; n + 1];
    for q in queries {
        d[q[0] as usize] += 1;
        d[q[1] as usize + 1] -= 1;
    }
    let mut s = 0;
    for i in 0..n {
        s += d[i];
        if nums[i] > s {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        is_zero_array(vec![1, 0, 1], vec![vec![0, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::is_zero_array;

    #[test]
    fn example1() {
        assert!(is_zero_array(vec![1, 0, 1], vec![vec![0, 2]]));
    }

    #[test]
    fn example2() {
        assert!(!is_zero_array(
            vec![4, 3, 2, 1],
            vec![vec![1, 3], vec![0, 2]]
        ));
    }
}
