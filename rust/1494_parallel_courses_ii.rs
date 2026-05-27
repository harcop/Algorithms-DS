/// LeetCode #1494 - Parallel Courses Ii
fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = n as usize;
    let mut pre = vec![0u32; n];
    for r in relations {
        pre[r[1] as usize - 1] |= 1u32 << (r[0] as usize - 1);
    }
    let full = (1u32 << n) - 1;
    let inf = i32::MAX / 4;
    let mut dp = vec![inf; 1 << n];
    dp[0] = 0;
    for mask in 0..(1 << n) {
        if dp[mask] == inf { continue; }
        let mut avail = 0u32;
        for i in 0..n {
            if mask & (1 << i) == 0 && (pre[i] & mask as u32) == pre[i] {
                avail |= 1u32 << i;
            }
        }
        let mut sub = avail;
        while sub > 0 {
            if sub.count_ones() as i32 <= k {
                dp[mask | sub as usize] = dp[mask | sub as usize].min(dp[mask] + 1);
            }
            sub = (sub - 1) & avail;
        }
    }
    if dp[full as usize] == inf { -1 } else { dp[full as usize] }
}
fn main() { println!("{}", min_number_of_semesters(4, vec![vec![2,1],vec![3,1],vec![1,4]], 2)); }
#[cfg(test)]
mod tests {
    use super::min_number_of_semesters;
    #[test]
    fn example_one() { assert_eq!(min_number_of_semesters(4, vec![vec![2,1],vec![3,1],vec![1,4]], 2), 3); }
    #[test]
    fn example_two() { assert_eq!(min_number_of_semesters(5, vec![vec![2,1],vec![3,1],vec![4,1],vec![1,5]], 2), 4); }
}