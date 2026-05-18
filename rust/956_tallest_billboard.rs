/// LeetCode #956 - Tallest Billboard
use std::collections::HashMap;

fn tallest_billboard(rods: Vec<i32>) -> i32 {
    let mut dp: HashMap<i32, i32> = HashMap::new();
    dp.insert(0, 0);
    for &r in &rods {
        let mut next = dp.clone();
        for (&diff, &h) in &dp {
            next.entry(diff + r)
                .and_modify(|v| *v = (*v).max(h))
                .or_insert(h);
            let nd = (diff - r).abs();
            let nh = h + r.min(diff);
            next.entry(nd)
                .and_modify(|v| *v = (*v).max(nh))
                .or_insert(nh);
        }
        dp = next;
    }
    *dp.get(&0).unwrap_or(&0)
}

fn main() {
    println!("{}", tallest_billboard(vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::tallest_billboard;

    #[test]
    fn example_one() {
        assert_eq!(tallest_billboard(vec![1, 2]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(tallest_billboard(vec![1, 2, 3, 6]), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(tallest_billboard(vec![1, 2, 3, 4, 5, 6]), 10);
    }
}
