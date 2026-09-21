/// LeetCode #3851 - Maximum Requests Without Violating the Limit (premium)
use std::collections::{HashMap, VecDeque};

fn max_requests(requests: Vec<Vec<i32>>, k: i32, window: i32) -> i32 {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for r in &requests {
        g.entry(r[0]).or_default().push(r[1]);
    }
    let mut ans = requests.len() as i32;
    for ts in g.values_mut() {
        ts.sort_unstable();
        let mut kept: VecDeque<i32> = VecDeque::new();
        for &t in ts.iter() {
            while kept.front().map(|&front| t - front > window).unwrap_or(false) {
                kept.pop_front();
            }
            if kept.len() < k as usize {
                kept.push_back(t);
            } else {
                ans -= 1;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_requests(vec![vec![1, 1], vec![2, 1], vec![1, 7], vec![2, 8]], 1, 4)
    );
}

#[cfg(test)]
mod tests {
    use super::max_requests;

    #[test]
    fn example1() {
        assert_eq!(
            max_requests(vec![vec![1, 1], vec![2, 1], vec![1, 7], vec![2, 8]], 1, 4),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_requests(vec![vec![1, 2], vec![1, 5], vec![1, 2], vec![1, 6]], 2, 5),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_requests(vec![vec![1, 1], vec![2, 5], vec![1, 2], vec![3, 9]], 1, 1),
            3
        );
    }
}
