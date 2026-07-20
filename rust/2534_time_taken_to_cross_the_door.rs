/// LeetCode #2534 - Time Taken to Cross the Door
use std::collections::VecDeque;

fn time_taken(arrival: Vec<i32>, state: Vec<i32>) -> Vec<i32> {
    let n = arrival.len();
    let mut q = [VecDeque::new(), VecDeque::new()];
    let mut t = 0i32;
    let mut i = 0usize;
    let mut st = 1usize;
    let mut ans = vec![0i32; n];

    while i < n || !q[0].is_empty() || !q[1].is_empty() {
        while i < n && arrival[i] <= t {
            q[state[i] as usize].push_back(i);
            i += 1;
        }
        if !q[0].is_empty() && !q[1].is_empty() {
            ans[q[st].pop_front().unwrap()] = t;
        } else if !q[0].is_empty() || !q[1].is_empty() {
            st = if q[0].is_empty() { 1 } else { 0 };
            ans[q[st].pop_front().unwrap()] = t;
        } else {
            st = 1;
        }
        t += 1;
    }
    ans
}

fn main() {
    println!("{:?}", time_taken(vec![0, 1, 1, 2, 4], vec![0, 1, 0, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::time_taken;

    #[test]
    fn example_one() {
        assert_eq!(
            time_taken(vec![0, 1, 1, 2, 4], vec![0, 1, 0, 0, 1]),
            vec![0, 3, 1, 2, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(time_taken(vec![0, 0, 0], vec![1, 0, 1]), vec![0, 2, 1]);
    }
}
