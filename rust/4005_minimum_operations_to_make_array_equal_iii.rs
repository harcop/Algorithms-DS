/// LeetCode #4005 - Minimum Operations to Make Array Equal III
use std::collections::HashMap;

fn divisors(x: i64) -> Vec<i64> {
    let mut d = Vec::new();
    let mut i = 1i64;
    while i * i <= x {
        if x % i == 0 {
            d.push(i);
            if i != x / i {
                d.push(x / i);
            }
        }
        i += 1;
    }
    d
}

fn min_operations(nums: Vec<i32>) -> i32 {
    let mut freq: HashMap<i64, i64> = HashMap::new();
    for x in nums {
        *freq.entry(x as i64).or_insert(0) += 1;
    }
    if freq.keys().all(|&k| k == 1) {
        return 0;
    }
    let n: i64 = freq.values().sum();
    let mut divisible_by: HashMap<i64, i64> = HashMap::new();
    for (&x, &f) in &freq {
        if x == 1 {
            continue;
        }
        for d in divisors(x) {
            if freq.contains_key(&d) {
                *divisible_by.entry(d).or_insert(0) += f;
            }
        }
    }
    let mut best = n;
    for (&v, _) in &freq {
        if v == 1 {
            continue;
        }
        let mut divide_v = 0i64;
        for d in divisors(v) {
            if let Some(&f) = freq.get(&d) {
                divide_v += f;
            }
        }
        let multiples = *divisible_by.get(&v).unwrap_or(&0);
        best = best.max(multiples + divide_v);
    }
    (2 * n - best) as i32
}

fn main() {
    println!("{}", min_operations(vec![6, 12, 8]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![6, 12, 8]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![5, 15, 20]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![7, 7, 7]), 0);
    }

    #[test]
    fn all_ones() {
        assert_eq!(min_operations(vec![1, 1, 1]), 0);
    }
}
