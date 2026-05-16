/// LeetCode #823 - Binary Trees With Factors
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
    let mut arr = arr;
    arr.sort_unstable();
    let mut dp: HashMap<i32, i64> = HashMap::new();
    for &x in &arr {
        let mut ways = 0i64;
        for &y in &arr {
            if y * y > x {
                break;
            }
            if x % y == 0 {
                let z = x / y;
                if let (Some(&a), Some(&b)) = (dp.get(&y), dp.get(&z)) {
                    let mut add = a * b % MOD;
                    if y != z {
                        add = add * 2 % MOD;
                    }
                    ways = (ways + add) % MOD;
                }
            }
        }
        dp.insert(x, (1 + ways) % MOD);
    }
    (dp.values().sum::<i64>() % MOD) as i32
}

fn main() {
    println!("{}", num_factored_binary_trees(vec![2, 7]));
}

#[cfg(test)]
mod tests {
    use super::num_factored_binary_trees;

    #[test]
    fn example_one() {
        assert_eq!(num_factored_binary_trees(vec![2, 7]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_factored_binary_trees(vec![2, 4]), 3);
    }
}
