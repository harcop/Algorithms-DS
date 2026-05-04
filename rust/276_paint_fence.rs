/// LeetCode #276 - Paint Fence
fn num_ways(n: i32, k: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return k;
    }
    let mut same = k;
    let mut diff = k * (k - 1);
    for _ in 2..n {
        let new_same = diff;
        let new_diff = (same + diff) * (k - 1);
        same = new_same;
        diff = new_diff;
    }
    (same + diff) as i32
}

fn main() {
    println!("{}", num_ways(3, 2));
}

#[cfg(test)]
mod tests {
    use super::num_ways;

    #[test]
    fn example_one() {
        assert_eq!(num_ways(3, 2), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_ways(1, 1), 1);
    }
}
