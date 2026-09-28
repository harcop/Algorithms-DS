/// LeetCode #3991 - Sort Array Using Prefix Reversals
use std::collections::{HashSet, VecDeque};

fn sort_array(nums: Vec<i32>, pre: Vec<i32>) -> i32 {
    let n = nums.len();
    let target: i32 = (0..n as i32).fold(0, |acc, v| acc * 8 + v);
    let start = nums.iter().fold(0, |acc, &v| acc * 8 + v);
    if start == target {
        return 0;
    }
    let mut vis = HashSet::new();
    vis.insert(start);
    let mut q = VecDeque::new();
    q.push_back((nums, 0));
    while let Some((state, dist)) = q.pop_front() {
        let nd = dist + 1;
        for &x in &pre {
            let x = x as usize;
            let mut nxt = state.clone();
            nxt[..x].reverse();
            let key = nxt.iter().fold(0, |acc, &v| acc * 8 + v);
            if key == target {
                return nd;
            }
            if vis.insert(key) {
                q.push_back((nxt, nd));
            }
        }
    }
    -1
}

fn main() {
    println!("{}", sort_array(vec![2, 0, 1], vec![2, 3]));
}

#[cfg(test)]
mod tests {
    use super::sort_array;

    #[test]
    fn example1() {
        assert_eq!(sort_array(vec![2, 0, 1], vec![2, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(sort_array(vec![1, 0, 2], vec![1, 3]), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(sort_array(vec![0, 1], vec![2]), 0);
    }
}
