/// LeetCode #1442 - Count Triplets That Can Form Two Arrays Of Equal Xor
fn count_triplets(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut pre = vec![0i32; n + 1];
    for i in 0..n { pre[i + 1] = pre[i] ^ arr[i]; }
    let mut ans = 0;
    for i in 0..=n {
        for k in i + 2..=n {
            if pre[i] == pre[k] { ans += (k - i - 1) as i32; }
        }
    }
    ans
}
fn main() { println!("{}", count_triplets(vec![2, 3, 1, 6, 7])); }
#[cfg(test)]
mod tests {
    use super::count_triplets;
    #[test]
    fn example_one() { assert_eq!(count_triplets(vec![2, 3, 1, 6, 7]), 4); }
    #[test]
    fn example_two() { assert_eq!(count_triplets(vec![1, 1, 1, 1, 1]), 10); }
}