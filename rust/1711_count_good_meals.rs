/// LeetCode #1711 - Count Good Meals
const MOD: i64 = 1_000_000_007;

fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut cnt = HashMap::new();
    let mut ans = 0i64;
    for x in deliciousness {
        let mut p = 1i64;
        while p <= 1 << 21 {
            if let Some(&c) = cnt.get(&(p - x as i64)) {
                ans = (ans + c) % MOD;
            }
            p <<= 1;
        }
        *cnt.entry(x as i64).or_insert(0i64) += 1;
    }
    ans as i32
}
fn main() { println!("{}", count_pairs(vec![1,3,5,7,9])); }
#[cfg(test)]
mod tests {
    use super::count_pairs;
    #[test]
    fn example_one() { assert_eq!(count_pairs(vec![1,3,5,7,9]), 4); }
}