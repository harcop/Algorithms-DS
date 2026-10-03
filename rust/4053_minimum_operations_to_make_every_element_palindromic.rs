/// LeetCode #4053 - Minimum Operations to Make Every Element Palindromic
fn min_operations(nums: Vec<i32>) -> i64 {
    let pals = palindromes();
    let mut ans = 0i64;
    for x in nums {
        let x = x as i64;
        let p = &pals[(x & 1) as usize];
        let i = p.partition_point(|&v| v < x);
        let mut best = i64::MAX;
        if i < p.len() {
            best = p[i] - x;
        }
        if i > 0 {
            best = best.min(x - p[i - 1]);
        }
        ans += best / 2;
    }
    ans
}

fn palindromes() -> [Vec<i64>; 2] {
    let mut ps = [Vec::new(), Vec::new()];
    for i in 1..=100_000i64 {
        let s = i.to_string();
        let rev: String = s.chars().rev().collect();
        let even: i64 = format!("{s}{rev}").parse().unwrap();
        ps[(even & 1) as usize].push(even);
        let body: String = s.chars().rev().skip(1).collect();
        let odd: i64 = format!("{s}{body}").parse().unwrap();
        ps[(odd & 1) as usize].push(odd);
    }
    ps[0].sort_unstable();
    ps[1].sort_unstable();
    ps
}

fn main() {
    println!("{}", min_operations(vec![10, 12, 14, 16]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![10, 12, 14, 16]), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![9, 10, 11, 10]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![125]), 2);
    }
}
