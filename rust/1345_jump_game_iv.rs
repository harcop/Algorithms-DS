/// LeetCode #1345 - Jump Game Iv

use std::collections::{HashMap, VecDeque};

fn min_jumps(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }
    let mut val_to_idx: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, &v) in arr.iter().enumerate() {
        val_to_idx.entry(v).or_default().push(i);
    }
    let mut dist = vec![i32::MAX; n];
    dist[0] = 0;
    let mut q = VecDeque::from([0usize]);
    while let Some(i) = q.pop_front() {
        if i == n - 1 {
            return dist[i];
        }
        for j in [i.wrapping_sub(1), i + 1] {
            if j < n && dist[j] == i32::MAX {
                dist[j] = dist[i] + 1;
                q.push_back(j);
            }
        }
        if let Some(indices) = val_to_idx.get_mut(&arr[i]) {
            while let Some(j) = indices.pop() {
                if j != i && dist[j] == i32::MAX {
                    dist[j] = dist[i] + 1;
                    q.push_back(j);
                }
            }
        }
    }
    dist[n - 1]
}

fn main() {
    println!("{}", min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]));
}

#[cfg(test)]
mod tests {
    use super::min_jumps;

    #[test]
    fn example_one() {
        assert_eq!(min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_jumps(vec![7]), 0);
    }
}
