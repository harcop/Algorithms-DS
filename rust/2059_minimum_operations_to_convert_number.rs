/// LeetCode #2059 - Minimum Operations to Convert Number
use std::collections::VecDeque;

fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
    let mut vis = vec![false; 1001];
    let mut q = VecDeque::new();
    q.push_back((start, 0i32));

    while let Some((x, step)) = q.pop_front() {
        for &num in &nums {
            for nx in [x + num, x - num, x ^ num] {
                if nx == goal {
                    return step + 1;
                }
                if (0..=1000).contains(&nx) && !vis[nx as usize] {
                    vis[nx as usize] = true;
                    q.push_back((nx, step + 1));
                }
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        minimum_operations(vec![2, 4, 12], 2, 12)
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example_one() {
        assert_eq!(minimum_operations(vec![2, 4, 12], 2, 12), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_operations(vec![3, 5, 7], 0, -4), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_operations(vec![2, 8, 16], 0, 1), -1);
    }
}
