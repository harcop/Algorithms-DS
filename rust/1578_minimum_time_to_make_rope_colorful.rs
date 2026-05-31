/// LeetCode #1578 - Minimum Time To Make Rope Colorful
fn min_cost(s: String, cost: Vec<i32>) -> i32 {
    let b = s.as_bytes();
    let mut ans = 0i32;
    let mut i = 0usize;
    while i < b.len() {
        let mut j = i;
        let mut mx = cost[i];
        let mut sum = cost[i];
        while j + 1 < b.len() && b[j + 1] == b[i] {
            j += 1;
            sum += cost[j];
            mx = mx.max(cost[j]);
        }
        if j > i { ans += sum - mx; }
        i = j + 1;
    }
    ans
}
fn main() { println!("{}", min_cost("abaac".into(), vec![1,2,3,4,5])); }
#[cfg(test)]
mod tests {
    use super::min_cost;
    #[test]
    fn example_one() { assert_eq!(min_cost("abaac".into(), vec![1,2,3,4,5]), 3); }
    #[test]
    fn example_two() { assert_eq!(min_cost("abc".into(), vec![1,2,3]), 0); }
}