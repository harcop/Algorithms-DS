/// LeetCode #2945 - Find Maximum Non-decreasing Array Length
fn find_maximum_length(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut s = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + nums[i] as i64;
    }
    let mut f = vec![0i32; n + 1];
    let mut pre = vec![0usize; n + 2];
    for i in 1..=n {
        pre[i] = pre[i].max(pre[i - 1]);
        f[i] = f[pre[i]] + 1;
        let target = s[i] * 2 - s[pre[i]];
        let j = s.partition_point(|&v| v < target);
        if j < pre.len() {
            pre[j] = pre[j].max(i);
        }
    }
    f[n]
}

fn main() {
    println!("{}", find_maximum_length(vec![5, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::find_maximum_length;

    #[test]
    fn example_one() {
        assert_eq!(find_maximum_length(vec![5, 2, 2]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_maximum_length(vec![1, 2, 3, 4]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_maximum_length(vec![4, 3, 2, 6]), 3);
    }
}
