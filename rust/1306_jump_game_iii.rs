/// LeetCode #1306 - Jump Game III
use std::collections::VecDeque;

fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    let n = arr.len();
    let mut seen = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(start as usize);
    seen[start as usize] = true;
    while let Some(i) = q.pop_front() {
        if arr[i] == 0 {
            return true;
        }
        for ni in [i as i32 + arr[i], i as i32 - arr[i]] {
            let ni = ni as usize;
            if ni < n && !seen[ni] {
                seen[ni] = true;
                q.push_back(ni);
            }
        }
    }
    false
}

fn main() {
    println!("{}", can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
}

#[cfg(test)]
mod tests {
    use super::can_reach;

    #[test]
    fn example_one() {
        assert!(can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
    }

    #[test]
    fn example_two() {
        assert!(can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
    }
}
