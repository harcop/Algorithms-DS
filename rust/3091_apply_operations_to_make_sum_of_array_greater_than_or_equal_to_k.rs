/// LeetCode #3091 - Apply Operations to Make Sum of Array Greater Than or Equal to k
fn min_operations(k: i32) -> i32 {
    // Optimal: increment the single element to some value x, then duplicate it enough times.
    // If final array has m copies of x (approx), sum = m*x >= k, ops = (x-1) + (m-1).
    // Minimize (x-1)+(m-1) with m*x >= k.
    let mut best = k - 1; // all increments on single element
    for x in 1..=k {
        let m = (k + x - 1) / x; // ceil(k/x)
        best = best.min((x - 1) + (m - 1));
    }
    best
}

fn main() {
    println!("{}", min_operations(11));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(11), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(1), 0);
    }
}
