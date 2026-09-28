/// LeetCode #3994 - Minimum Adjacent Swaps to Partition Array
fn min_swaps(nums: Vec<i32>, a: i32, b: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut seen = [0i64; 3];
    let mut inv = 0i64;
    for x in nums {
        let t = if x < a {
            0
        } else if x <= b {
            1
        } else {
            2
        };
        for higher in (t + 1)..3 {
            inv += seen[higher];
        }
        seen[t] += 1;
    }
    (inv % MOD) as i32
}

fn main() {
    println!("{}", min_swaps(vec![1, 3, 2, 4, 5, 6], 3, 4));
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    fn example1() {
        assert_eq!(min_swaps(vec![1, 3, 2, 4, 5, 6], 3, 4), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_swaps(vec![9, 7, 5, 3], 4, 8), 5);
    }

    #[test]
    fn example3() {
        assert_eq!(min_swaps(vec![3, 7, 5, 9], 4, 8), 0);
    }
}
