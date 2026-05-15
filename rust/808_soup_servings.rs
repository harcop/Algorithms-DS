/// LeetCode #808 - Soup Servings
use std::collections::HashMap;

fn soup_servings(n: i32) -> f64 {
    let n = (n + 24) / 25;
    if n > 200 {
        return 1.0;
    }
    let mut memo: HashMap<(i32, i32), f64> = HashMap::new();

    fn dp(a: i32, b: i32, memo: &mut HashMap<(i32, i32), f64>) -> f64 {
        if a <= 0 && b <= 0 {
            return 0.5;
        }
        if a <= 0 {
            return 1.0;
        }
        if b <= 0 {
            return 0.0;
        }
        if let Some(&v) = memo.get(&(a, b)) {
            return v;
        }
        let mut p = 0.0;
        for (x, y) in [(4, 0), (3, 1), (2, 2), (1, 3), (0, 4)] {
            p += dp(a - x, b - y, memo);
        }
        let ans = p * 0.25;
        memo.insert((a, b), ans);
        ans
    }

    dp(n, n, &mut memo)
}

fn main() {
    println!("{}", soup_servings(50));
}

#[cfg(test)]
mod tests {
    use super::soup_servings;

    #[test]
    fn example_one() {
        let v = soup_servings(50);
        assert!((v - 0.625).abs() < 0.01, "got {}", v);
    }
}
