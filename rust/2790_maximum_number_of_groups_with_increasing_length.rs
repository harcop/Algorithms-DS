/// LeetCode #2790 - Maximum Number of Groups With Increasing Length
fn max_increasing_groups(mut usage_limits: Vec<i32>) -> i32 {
    usage_limits.sort_unstable();
    let (mut k, mut s) = (0i64, 0i64);
    for &x in &usage_limits {
        s += x as i64;
        if s > k {
            k += 1;
            s -= k;
        }
    }
    k as i32
}

fn main() {
    println!("{}", max_increasing_groups(vec![1, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_increasing_groups;

    #[test]
    fn example_one() {
        assert_eq!(max_increasing_groups(vec![1, 2, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_increasing_groups(vec![2, 1, 2]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_increasing_groups(vec![1, 1]), 1);
    }
}
