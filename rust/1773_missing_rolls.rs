/// LeetCode #1773 - Missing Rolls
fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let need = mean * (rolls.len() as i32 + n) - rolls.iter().sum::<i32>();
    if need < n || need > 6 * n {
        return vec![];
    }
    let mut ans = vec![1; n as usize];
    let mut rem = need - n;
    let mut i = 0usize;
    while rem > 0 {
        let add = rem.min(5);
        ans[i] += add;
        rem -= add;
        i += 1;
    }
    ans
}
fn main() { println!("{:?}", missing_rolls(vec![3, 2, 4, 3], 4, 2)); }
#[cfg(test)]
mod tests {
    use super::missing_rolls;
    #[test]
    fn example_one() { assert_eq!(missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6]); }
    #[test]
    fn example_two() { assert_eq!(missing_rolls(vec![1, 5, 6], 3, 4), vec![6, 1, 1, 1]); }
}
