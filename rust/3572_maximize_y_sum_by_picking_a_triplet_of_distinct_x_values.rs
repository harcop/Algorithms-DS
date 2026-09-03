/// LeetCode #3572 - Maximize Y-Sum by Picking a Triplet of Distinct X-Values
use std::collections::HashSet;

fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
    let mut arr: Vec<(i32, i32)> = x.into_iter().zip(y).collect();
    arr.sort_by(|a, b| b.1.cmp(&a.1));
    let mut vis = HashSet::new();
    let mut ans = 0;
    for (a, b) in arr {
        if vis.insert(a) {
            ans += b;
            if vis.len() == 3 {
                return ans;
            }
        }
    }
    -1
}

fn main() {
    println!("{}", max_sum_distinct_triplet(vec![1, 2, 1, 3, 2], vec![5, 3, 4, 6, 2]));
}

#[cfg(test)]
mod tests {
    use super::max_sum_distinct_triplet;

    #[test]
    fn example1() {
        assert_eq!(
            max_sum_distinct_triplet(vec![1, 2, 1, 3, 2], vec![5, 3, 4, 6, 2]),
            14
        );
    }

    #[test]
    fn example2() {
        assert_eq!(max_sum_distinct_triplet(vec![1, 2, 1, 2], vec![4, 5, 6, 7]), -1);
    }
}
