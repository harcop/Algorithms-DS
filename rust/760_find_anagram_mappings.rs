/// LeetCode #760 - Find Anagram Mappings
use std::collections::HashMap;

fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
    for (j, &x) in nums2.iter().enumerate() {
        pos.entry(x).or_default().push(j);
    }
    let mut out = Vec::with_capacity(nums1.len());
    for x in nums1 {
        let v = pos.get_mut(&x).unwrap();
        let j = v.pop().unwrap();
        out.push(j as i32);
    }
    out
}

fn main() {
    println!("{:?}", anagram_mappings(vec![12, 28, 46], vec![46, 12, 28]));
}

#[cfg(test)]
mod tests {
    use super::anagram_mappings;

    #[test]
    fn example_one() {
        let nums1 = vec![12, 28, 46];
        let nums2 = vec![46, 12, 28];
        let m = anagram_mappings(nums1.clone(), nums2.clone());
        for i in 0..nums1.len() {
            assert_eq!(nums1[i], nums2[m[i] as usize]);
        }
    }
}
