/// LeetCode #3739 - Count Subarrays With Majority Element II
fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
    let n = nums.len();
    let mut tree = vec![0i32; 2 * n + 2];
    let m = 2 * n + 1;
    let update = |tree: &mut [i32], mut x: usize| {
        while x <= m {
            tree[x] += 1;
            x += x & x.wrapping_neg();
        }
    };
    let query = |tree: &[i32], mut x: usize| {
        let mut s = 0i32;
        while x > 0 {
            s += tree[x];
            x -= x & x.wrapping_neg();
        }
        s
    };
    let mut s = n + 1;
    update(&mut tree, s);
    let mut ans = 0i64;
    for &x in &nums {
        if x == target {
            s += 1;
        } else {
            s -= 1;
        }
        ans += query(&tree, s - 1) as i64;
        update(&mut tree, s);
    }
    ans
}

fn main() {
    println!("{}", count_majority_subarrays(vec![1, 2, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::count_majority_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(count_majority_subarrays(vec![1, 1, 1, 1], 1), 10);
    }

    #[test]
    fn example3() {
        assert_eq!(count_majority_subarrays(vec![1, 2, 3], 4), 0);
    }
}
