/// LeetCode #2580 - Count Ways to Group Overlapping Ranges
const MOD: i32 = 1_000_000_007;

fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
    ranges.sort_unstable_by_key(|r| r[0]);
    let mut mx = -1;
    let mut ans = 1i32;
    for e in ranges {
        if e[0] > mx {
            ans = (ans * 2) % MOD;
        }
        mx = mx.max(e[1]);
    }
    ans
}

fn main() {
    println!("{}", count_ways(vec![vec![6, 10], vec![5, 15]]));
}

#[cfg(test)]
mod tests {
    use super::count_ways;

    #[test]
    fn example_one() {
        assert_eq!(count_ways(vec![vec![6, 10], vec![5, 15]]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_ways(vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]]),
            4
        );
    }
}
