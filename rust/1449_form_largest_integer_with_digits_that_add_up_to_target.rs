/// LeetCode #1449 - Form Largest Integer With Digits That Add Up To Target
fn largest_number(cost: Vec<i32>, target: i32) -> String {
    let t = target as usize;
    let mut dp: Vec<Option<String>> = vec![None; t + 1];
    dp[0] = Some(String::new());
    for d in 1..=9 {
        let c = cost[d - 1] as usize;
        for s in c..=t {
            if let Some(prev) = dp[s - c].clone() {
                let cand = format!("{}{}", prev, d);
                dp[s] = match dp[s].take() {
                    None => Some(cand),
                    Some(ex) if cand.len() > ex.len() || (cand.len() == ex.len() && cand > ex) => Some(cand),
                    Some(ex) => Some(ex),
                };
            }
        }
    }
    dp[t].clone().unwrap_or_else(|| "0".to_string())
}
fn main() { println!("{}", largest_number(vec![1,100,1,1,1,1,1,1,1], 3)); }
#[cfg(test)]
mod tests {
    use super::largest_number;
    #[test]
    fn example_one() { assert_eq!(largest_number(vec![1,100,1,1,1,1,1,1,1], 3), "999"); }
    #[test]
    fn example_two() {
        let ans = largest_number(vec![1,1,1,1,1,1,1,1,1], 5000);
        assert_eq!(ans.len(), 5000);
        assert!(ans.chars().all(|c| c == '9'));
    }
}