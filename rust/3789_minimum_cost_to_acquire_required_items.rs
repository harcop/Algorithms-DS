/// LeetCode #3789 - Minimum Cost to Acquire Required Items
fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
    let a = need1 as i64 * cost1 as i64 + need2 as i64 * cost2 as i64;
    let b = cost_both as i64 * need1.max(need2) as i64;
    let mn = need1.min(need2);
    let c = cost_both as i64 * mn as i64
        + (need1 - mn) as i64 * cost1 as i64
        + (need2 - mn) as i64 * cost2 as i64;
    a.min(b).min(c)
}

fn main() {
    println!("{}", minimum_cost(3, 2, 1, 3, 2));
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example1() {
        assert_eq!(minimum_cost(3, 2, 1, 3, 2), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_cost(5, 4, 15, 2, 3), 22);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_cost(5, 4, 15, 0, 0), 0);
    }
}
