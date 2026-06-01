/// LeetCode #1703 - Minimum Adjacent Swaps For K Consecutive Ones
fn min_swaps(nums: Vec<i32>, k: i32) -> i32 {
    let mut ones = vec![];
    for (i, &x) in nums.iter().enumerate() {
        if x == 1 { ones.push(i as i32); }
    }
    if ones.len() < k as usize { return -1; }
    let k = k as usize;
    let mut best = i32::MAX;
    let mut cur = 0i32;
    for i in 0..k {
        cur += ones[i] - ones[0] - i as i32;
    }
    best = best.min(cur);
    for i in 1..=ones.len() - k {
        cur -= ones[i + k - 1] - ones[i - 1] - (k as i32 - 1);
        cur += ones[i + k - 1] - ones[i] - (k as i32 - 1);
        let nc = (0..k).map(|j| (ones[i + j] - (ones[i] + j as i32)).abs()).sum::<i32>();
        let mut nc = 0i32;
        for j in 0..k {
            nc += ones[i + j] - (ones[i - 1] + 1 + j as i32);
        }
        best = best.min(nc);
    }
    best
}
fn main() { println!("{}", min_swaps(vec![1,0,0,1,0,1], 2)); }
#[cfg(test)]
mod tests {
    use super::min_swaps;
    #[test]
    fn example_one() { assert_eq!(min_swaps(vec![1,0,0,1,0,1], 2), 1); }
}