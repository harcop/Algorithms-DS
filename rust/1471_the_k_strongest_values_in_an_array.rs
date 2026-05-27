/// LeetCode #1471 - The K Strongest Values In An Array
fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut a = arr.clone();
    a.sort_unstable();
    let mid = a[(a.len() - 1) / 2];
    a.sort_by_key(|&x| (x - mid).abs());
    a.reverse();
    a.into_iter().take(k as usize).collect()
}
fn main() { println!("{:?}", get_strongest(vec![1,2,3,4,5], 2)); }
#[cfg(test)]
mod tests {
    use super::get_strongest;
    #[test]
    fn example_one() { assert_eq!(get_strongest(vec![1,2,3,4,5], 2), vec![5,1]); }
    #[test]
    fn example_two() { assert_eq!(get_strongest(vec![1,1,3,5,5], 2), vec![5,5]); }
}