/// LeetCode #4025 - Minimize the Maximum Waiting Time at Synchronized Traffic Lights
fn min_penalty(period: i32, lights: Vec<i32>, arrival_time: Vec<i32>) -> i32 {
    let mx = lights.into_iter().max().unwrap_or(0);
    let mut ans = 0;
    for x in arrival_time {
        let r = x % period;
        if r >= mx {
            ans = ans.max(period - r);
        }
    }
    ans
}

fn main() {
    println!("{}", min_penalty(8, vec![2, 3], vec![2, 5, 8, 11]));
}

#[cfg(test)]
mod tests {
    use super::min_penalty;

    #[test]
    fn example1() {
        assert_eq!(min_penalty(8, vec![2, 3], vec![2, 5, 8, 11]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(min_penalty(10, vec![3, 6, 8], vec![4, 9, 15]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_penalty(5, vec![2], vec![2, 3, 4, 5, 6]), 3);
    }
}
