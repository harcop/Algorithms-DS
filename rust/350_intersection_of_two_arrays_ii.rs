/// LeetCode #350 - Intersection of Two Arrays II
use std::collections::HashMap;

fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut c = HashMap::new();
    for x in nums1 {
        *c.entry(x).or_insert(0) += 1;
    }
    let mut out = vec![];
    for x in nums2 {
        if let Some(v) = c.get_mut(&x) {
            if *v > 0 {
                out.push(x);
                *v -= 1;
            }
        }
    }
    out
}

fn main() {
    println!("{:?}", intersect(vec![1,2,2,1], vec![2,2]));
}

#[cfg(test)]
mod tests {
    use super::intersect;

    #[test]
    fn example_one() {
        let mut v = intersect(vec![1,2,2,1], vec![2,2]);
        v.sort_unstable();
        assert_eq!(v, vec![2,2]);
    }
}
