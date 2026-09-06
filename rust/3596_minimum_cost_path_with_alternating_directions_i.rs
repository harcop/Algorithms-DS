/// LeetCode #3596 - Minimum Cost Path with Alternating Directions I
fn min_cost(m: i32, n: i32) -> i32 {
    if m == 1 && n == 1 {
        1
    } else if (m == 1 && n == 2) || (m == 2 && n == 1) {
        3
    } else {
        -1
    }
}

fn main() {
    println!("{}", min_cost(1, 1));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost(1, 1), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost(2, 1), 3);
    }

    #[test]
    fn one_by_two() {
        assert_eq!(min_cost(1, 2), 3);
    }

    #[test]
    fn impossible() {
        assert_eq!(min_cost(2, 2), -1);
    }
}
