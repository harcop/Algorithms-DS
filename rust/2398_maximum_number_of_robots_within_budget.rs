/// LeetCode #2398 - Maximum Number of Robots Within Budget
use std::collections::VecDeque;

fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
    let n = charge_times.len();
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut s: i64 = 0;
    let mut ans = 0i32;
    let mut l = 0;
    for r in 0..n {
        s += running_costs[r] as i64;
        while !q.is_empty() && charge_times[*q.back().unwrap()] <= charge_times[r] {
            q.pop_back();
        }
        q.push_back(r);
        while !q.is_empty()
            && (r as i64 - l as i64 + 1) * s + charge_times[*q.front().unwrap()] as i64 > budget
        {
            if *q.front().unwrap() == l {
                q.pop_front();
            }
            s -= running_costs[l] as i64;
            l += 1;
        }
        if r >= l {
            ans = ans.max((r - l + 1) as i32);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_robots(vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25)
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_robots;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_robots(vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_robots(vec![11, 12, 19], vec![10, 8, 7], 19), 0);
    }
}
