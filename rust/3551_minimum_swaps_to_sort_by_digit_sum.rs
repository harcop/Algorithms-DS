/// LeetCode #3551 - Minimum Swaps to Sort by Digit Sum
fn digit_sum(mut x: i32) -> i32 {
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn min_swaps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut arr: Vec<(i32, i32)> = nums.iter().map(|&x| (digit_sum(x), x)).collect();
    arr.sort_unstable();
    let mut pos = std::collections::HashMap::new();
    for (i, &(_, x)) in arr.iter().enumerate() {
        pos.insert(x, i);
    }
    let mut ans = n as i32;
    let mut vis = vec![false; n];
    for i in 0..n {
        if !vis[i] {
            ans -= 1;
            let mut j = i;
            while !vis[j] {
                vis[j] = true;
                j = pos[&nums[j]];
            }
        }
    }
    ans
}

fn main() {
    println!("{}", min_swaps(vec![37, 100]));
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    fn example1() {
        assert_eq!(min_swaps(vec![37, 100]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_swaps(vec![22, 14, 33, 7]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_swaps(vec![18, 43, 34, 16]), 2);
    }
}
