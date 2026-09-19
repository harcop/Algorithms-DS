/// LeetCode #3814 - Maximum Capacity Within Budget
use std::collections::BTreeSet;

fn max_capacity(costs: Vec<i32>, capacity: Vec<i32>, budget: i32) -> i32 {
    let mut arr: Vec<(i32, i32)> = costs
        .into_iter()
        .zip(capacity)
        .filter(|&(a, _)| a < budget)
        .collect();
    if arr.is_empty() {
        return 0;
    }
    arr.sort_unstable();
    let mut remain: BTreeSet<(i32, usize)> = BTreeSet::new();
    for (i, &(_, b)) in arr.iter().enumerate() {
        remain.insert((b, i));
    }
    let mut i = 0;
    let mut j = arr.len() - 1;
    let mut ans = remain.last().unwrap().0;
    while i < j {
        remain.remove(&(arr[i].1, i));
        while i < j && arr[i].0 + arr[j].0 >= budget {
            remain.remove(&(arr[j].1, j));
            j -= 1;
        }
        if let Some(&(b, _)) = remain.last() {
            ans = ans.max(arr[i].1 + b);
        }
        i += 1;
    }
    ans
}

fn main() {
    println!("{}", max_capacity(vec![4, 8, 5, 3], vec![1, 5, 2, 7], 8));
}

#[cfg(test)]
mod tests {
    use super::max_capacity;

    #[test]
    fn example1() {
        assert_eq!(max_capacity(vec![4, 8, 5, 3], vec![1, 5, 2, 7], 8), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(max_capacity(vec![3, 5, 7, 4], vec![2, 4, 3, 6], 7), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(max_capacity(vec![2, 2, 2], vec![3, 5, 4], 5), 9);
    }
}
