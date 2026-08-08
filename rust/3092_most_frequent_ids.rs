/// LeetCode #3092 - Most Frequent IDs
use std::collections::{BinaryHeap, HashMap};

fn most_frequent_ids(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
    let mut count: HashMap<i32, i64> = HashMap::new();
    let mut heap: BinaryHeap<(i64, i32)> = BinaryHeap::new();
    let mut ans = Vec::with_capacity(nums.len());

    for (id, delta) in nums.into_iter().zip(freq) {
        let entry = count.entry(id).or_insert(0);
        *entry += delta as i64;
        let cur = *entry;
        heap.push((cur, id));

        while let Some(&(top_freq, top_id)) = heap.peek() {
            if count.get(&top_id).copied().unwrap_or(0) == top_freq {
                break;
            }
            heap.pop();
        }

        ans.push(heap.peek().map(|&(f, _)| f).unwrap_or(0));
    }

    ans
}

fn main() {
    println!("{:?}", most_frequent_ids(vec![2, 3, 2, 1], vec![3, 2, -3, 1]));
}

#[cfg(test)]
mod tests {
    use super::most_frequent_ids;

    #[test]
    fn example1() {
        assert_eq!(
            most_frequent_ids(vec![2, 3, 2, 1], vec![3, 2, -3, 1]),
            vec![3, 3, 2, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            most_frequent_ids(vec![5, 5, 3], vec![2, -2, 1]),
            vec![2, 0, 1]
        );
    }
}
