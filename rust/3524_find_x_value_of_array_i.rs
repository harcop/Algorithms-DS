/// LeetCode #3524 - Find X Value of Array I
fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
    let k = k as usize;
    let mut ans = vec![0i64; k];
    let mut prev = vec![0i64; k];
    for &x in &nums {
        let m = (x as i64 % k as i64) as usize;
        let mut cur = vec![0i64; k];
        cur[m] += 1;
        for r in 0..k {
            if prev[r] > 0 {
                cur[(r * m) % k] += prev[r];
            }
        }
        for r in 0..k {
            ans[r] += cur[r];
        }
        prev = cur;
    }
    ans
}

fn main() {
    println!("{:?}", result_array(vec![1, 2, 3, 4, 5], 3));
}

#[cfg(test)]
mod tests {
    use super::result_array;

    #[test]
    fn example1() {
        assert_eq!(result_array(vec![1, 2, 3, 4, 5], 3), vec![9, 2, 4]);
    }

    #[test]
    fn example2() {
        assert_eq!(result_array(vec![1, 2, 4, 8, 16, 32], 4), vec![18, 1, 2, 0]);
    }

    #[test]
    fn example3() {
        assert_eq!(result_array(vec![1, 1, 2, 1, 1], 2), vec![9, 6]);
    }
}
