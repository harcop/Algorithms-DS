/// LeetCode #2064 - Minimized Maximum of Products Distributed to Any Store
fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let n = n as i64;
    let check = |x: i64| -> bool {
        quantities
            .iter()
            .map(|&v| (v as i64 + x - 1) / x)
            .sum::<i64>()
            <= n
    };

    let mut lo = 1i64;
    let mut hi = 1_000_000i64;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if check(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", minimized_maximum(6, vec![11, 6]));
}

#[cfg(test)]
mod tests {
    use super::minimized_maximum;

    #[test]
    fn example_one() {
        assert_eq!(minimized_maximum(6, vec![11, 6]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimized_maximum(7, vec![15, 10, 10]), 5);
    }
}
