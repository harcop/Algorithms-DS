/// LeetCode #3351 - Sum of Good Subsequences
use std::collections::HashMap;

fn sum_of_good_subsequences(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut f: HashMap<i32, i64> = HashMap::new();
    let mut g: HashMap<i32, i64> = HashMap::new();
    for x in nums {
        let fx = *f.get(&x).unwrap_or(&0);
        let gx = *g.get(&x).unwrap_or(&0);
        let fm = *f.get(&(x - 1)).unwrap_or(&0);
        let gm = *g.get(&(x - 1)).unwrap_or(&0);
        let fp = *f.get(&(x + 1)).unwrap_or(&0);
        let gp = *g.get(&(x + 1)).unwrap_or(&0);
        let nx = x as i64;
        f.insert(x, (fx + nx + fm + gm * nx + fp + gp * nx) % MOD);
        g.insert(x, (gx + 1 + gm + gp) % MOD);
    }
    (f.values().sum::<i64>() % MOD) as i32
}

fn main() {
    println!("{}", sum_of_good_subsequences(vec![1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::sum_of_good_subsequences;

    #[test]
    fn example1() {
        assert_eq!(sum_of_good_subsequences(vec![1, 2, 1]), 14);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_good_subsequences(vec![3, 4, 5]), 40);
    }
}
