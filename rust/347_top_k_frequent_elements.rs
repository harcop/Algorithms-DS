/// LeetCode #347 - Top K Frequent Elements
use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut m = HashMap::new();
    for x in nums {
        *m.entry(x).or_insert(0) += 1;
    }
    let mut v: Vec<(i32, i32)> = m.into_iter().collect();
    v.sort_unstable_by(|a,b| b.1.cmp(&a.1));
    v.into_iter().take(k as usize).map(|(x,_)| x).collect()
}

fn main() {
    println!("{:?}", top_k_frequent(vec![1,1,1,2,2,3], 2));
}

#[cfg(test)]
mod tests {
    use super::top_k_frequent;

    #[test]
    fn example_one() {
        let mut ans = top_k_frequent(vec![1,1,1,2,2,3], 2);
        ans.sort_unstable();
        assert_eq!(ans, vec![1,2]);
    }
}
