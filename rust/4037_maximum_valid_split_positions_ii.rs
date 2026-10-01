/// LeetCode #4037 - Maximum Valid Split Positions II
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
    let mut pre = arr[0];
    let mut suf = vec![0; m];
    suf[m - 1] = arr[m - 1];
    for i in (0..m - 1).rev() {
        suf[i] = igcd(suf[i + 1], arr[i]);
    }
    let mut ans = 0;
    for i in 0..m - 1 {
        if pre == suf[i + 1] {
            ans += 1;
        }
        pre = igcd(pre, arr[i + 1]);
    }
    ans
}

fn mark(arr: &[i32]) -> Vec<bool> {
    let n = arr.len();
    let mut pos = vec![false; n];
    pos[0] = true;
    let mut g = arr[0];
    for i in 1..n {
        let ng = igcd(g, arr[i]);
        pos[i] = ng != g;
        g = ng;
    }
    pos
}

fn max_valid_splits(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = score(&nums);
    let pos1 = mark(&nums);
    let mut rev = nums.clone();
    rev.reverse();
    let pos2 = mark(&rev);
    for i in 0..n {
        if pos1[i] || pos2[n - 1 - i] {
            let mut arr = Vec::with_capacity(n - 1);
            arr.extend_from_slice(&nums[..i]);
            arr.extend_from_slice(&nums[i + 1..]);
            ans = ans.max(score(&arr));
        }
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
