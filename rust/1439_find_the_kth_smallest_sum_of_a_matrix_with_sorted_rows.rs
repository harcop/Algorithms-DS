/// LeetCode #1439 - Find The Kth Smallest Sum Of A Matrix With Sorted Rows
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = mat.len();
    let mut cols = vec![0usize; m];
    let mut sum: i32 = mat.iter().enumerate().map(|(i, row)| row[cols[i]]).sum();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((sum, cols.clone())));
    let mut seen = std::collections::HashSet::new();
    seen.insert(cols);
    for _ in 0..k {
        let Reverse((s, c)) = heap.pop().unwrap();
        sum = s;
        for i in 0..m {
            if c[i] + 1 < mat[i].len() {
                let mut nc = c.clone();
                nc[i] += 1;
                if seen.insert(nc.clone()) {
                    let ns: i32 = mat.iter().enumerate().map(|(j, row)| row[nc[j]]).sum();
                    heap.push(Reverse((ns, nc)));
                }
            }
        }
    }
    sum
}

fn main() {
    println!("{}", kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 5));
}

#[cfg(test)]
mod tests {
    use super::kth_smallest;

    #[test]
    fn example_one() {
        assert_eq!(kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 5), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 9), 17);
    }
}

