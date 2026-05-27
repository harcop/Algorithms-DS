/// LeetCode #1467 - Probability Of A Two Boxes Having The Same Number Of Distinct Balls
fn get_probability(balls: Vec<i32>) -> f64 {
    let mut expanded = Vec::new();
    for (i, &c) in balls.iter().enumerate() {
        for _ in 0..c { expanded.push(i as i32); }
    }
    let total = expanded.len();
    let half = total / 2;
    let mut good = 0u64;
    let mut tot = 0u64;
    let n = total as u32;
    for mask in 0u64..(1u64 << n) {
        let mut b1 = 0usize;
        let mut s1 = std::collections::HashSet::new();
        let mut s2 = std::collections::HashSet::new();
        for i in 0..total {
            if (mask >> i) & 1 == 1 {
                b1 += 1;
                s1.insert(expanded[i]);
            } else {
                s2.insert(expanded[i]);
            }
        }
        if b1 != half { continue; }
        tot += 1;
        if s1.len() == s2.len() { good += 1; }
    }
    if tot == 0 { 0.0 } else { good as f64 / tot as f64 }
}
fn main() { println!("{}", get_probability(vec![1,1])); }
#[cfg(test)]
mod tests {
    use super::get_probability;
    #[test]
    fn example_one() { assert!((get_probability(vec![1,1]) - 1.0).abs() < 1e-5); }
    #[test]
    fn example_two() { assert!((get_probability(vec![2,1,1]) - 0.66667).abs() < 1e-3); }
}