/// LeetCode #954 - Array of Doubled Pairs
use std::collections::HashMap;

fn can_reorder_doubled(arr: Vec<i32>) -> bool {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for x in arr {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut keys: Vec<i32> = cnt.keys().copied().collect();
    keys.sort_unstable_by_key(|&k| k.abs());
    for k in keys {
        let c = *cnt.get(&k).unwrap_or(&0);
        if c == 0 {
            continue;
        }
        let partner = k.saturating_mul(2);
        let c2 = *cnt.get(&partner).unwrap_or(&0);
        if c2 < c {
            return false;
        }
        *cnt.get_mut(&k).unwrap() = 0;
        *cnt.entry(partner).or_insert(0) -= c;
    }
    true
}

fn main() {
    println!("{}", can_reorder_doubled(vec![4, -2, 2, -4]));
}

#[cfg(test)]
mod tests {
    use super::can_reorder_doubled;

    #[test]
    fn example_one() {
        assert!(can_reorder_doubled(vec![4, -2, 2, -4]));
    }

    #[test]
    fn example_two() {
        assert!(!can_reorder_doubled(vec![2, 1, 2, 6]));
    }
}
