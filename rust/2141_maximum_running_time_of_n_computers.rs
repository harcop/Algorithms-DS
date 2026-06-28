/// LeetCode #2141 - Maximum Running Time of N Computers
fn max_run_time(n: i32, batteries: Vec<i32>) -> i32 {
    let n = n as i64;
    let mut batteries: Vec<i64> = batteries.into_iter().map(|x| x as i64).collect();
    batteries.sort_unstable();

    let total: i64 = batteries.iter().sum();
    let mut low = 0i64;
    let mut high = total / n;

    while low < high {
        let mid = (low + high + 1) / 2;
        let supply: i64 = batteries.iter().map(|&b| b.min(mid)).sum();
        if supply >= n * mid {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    low as i32
}

fn main() {
    println!("{}", max_run_time(2, vec![3, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_run_time;

    #[test]
    fn example_one() {
        assert_eq!(max_run_time(2, vec![3, 3, 3]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_run_time(2, vec![1, 1, 1, 1]), 2);
    }
}
