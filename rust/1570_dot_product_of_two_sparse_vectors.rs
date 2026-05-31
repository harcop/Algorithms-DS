/// LeetCode #1570 - Dot Product Of Two Sparse Vectors
use std::collections::HashMap;

pub struct SparseVector {
    data: HashMap<i32, i32>,
}

impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let mut data = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            if v != 0 { data.insert(i as i32, v); }
        }
        SparseVector { data }
    }
    fn dot_product(&self, vec: &SparseVector) -> i32 {
        let (a, b) = if self.data.len() <= vec.data.len() {
            (&self.data, &vec.data)
        } else {
            (&vec.data, &self.data)
        };
        a.iter().map(|(i, v)| v * b.get(i).unwrap_or(&0)).sum()
    }
}
fn main() {
    let a = SparseVector::new(vec![1, 0, 0, 2, 3]);
    let b = SparseVector::new(vec![0, 3, 0, 4, 0]);
    println!("{}", a.dot_product(&b));
}
#[cfg(test)]
mod tests {
    use super::SparseVector;
    #[test]
    fn example_one() {
        let a = SparseVector::new(vec![1, 0, 0, 2, 3]);
        let b = SparseVector::new(vec![0, 3, 0, 4, 0]);
        assert_eq!(a.dot_product(&b), 8);
    }
}