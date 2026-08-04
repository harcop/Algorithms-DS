/// LeetCode #2967 - Minimum Cost to Make Array Equalindromic
fn build_palindromes() -> Vec<i64> {
    let mut ps = Vec::new();
    for i in 1..=100_000 {
        let s = i.to_string();
        let t1: String = s.chars().rev().collect();
        let t2: String = s[..s.len().saturating_sub(1)].chars().rev().collect();
        ps.push(format!("{s}{t1}").parse::<i64>().unwrap());
        ps.push(format!("{s}{t2}").parse::<i64>().unwrap());
    }
    ps.sort_unstable();
    ps.dedup();
    ps
}

fn minimum_cost(mut nums: Vec<i32>) -> i64 {
    let ps = build_palindromes();
    nums.sort_unstable();
    let median = nums[nums.len() / 2] as i64;
    let i = ps.partition_point(|&x| x < median);
    let cost = |x: i64| -> i64 { nums.iter().map(|&v| (v as i64 - x).abs()).sum() };
    let mut ans = i64::MAX;
    for j in (i as i32 - 1)..=(i as i32 + 1) {
        if j >= 0 && (j as usize) < ps.len() {
            ans = ans.min(cost(ps[j as usize]));
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_cost(vec![1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(minimum_cost(vec![1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_cost(vec![10, 12, 13, 14, 15]), 11);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_cost(vec![22, 33, 22, 33, 22]), 22);
    }
}
