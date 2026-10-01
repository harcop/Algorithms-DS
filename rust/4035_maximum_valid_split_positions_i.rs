/// LeetCode #4035 - Maximum Valid Split Positions I
fn igcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn score(arr: &[i32]) -> i32 {
    let m = arr.len();
    if m <= 1 {
        return 0;
    }
    let mut pre = vec![0; m];
    let mut suf = vec![0; m];
    pre[0] = arr[0];
    for i in 1..m {
        pre[i] = igcd(pre[i - 1], arr[i]);
    }
    suf[m - 1] = arr[m - 1];
    for i in (0..m - 1).rev() {
        suf[i] = igcd(suf[i + 1], arr[i]);
    }
    (0..m - 1).filter(|&i| pre[i] == suf[i + 1]).count() as i32
}

fn max_valid_splits(nums: Vec<i32>) -> i32 {
    let mut ans = score(&nums);
    let mut arr = nums.clone();
    for i in 0..nums.len() {
        arr.remove(i);
        ans = ans.max(score(&arr));
        arr.insert(i, nums[i]);
    }
    ans
}

fn main() {
    println!("{}", max_valid_splits(vec![10, 30, 15, 10]));
}

#[cfg(test)]
mod tests {
    use super::max_valid_splits;

    #[test]
    fn example1() {
        assert_eq!(max_valid_splits(vec![10, 30, 15, 10]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_valid_splits(vec![2, 10, 14]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(max_valid_splits(vec![2, 4]), 0);
    }
}
