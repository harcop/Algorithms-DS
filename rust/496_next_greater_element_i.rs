/// LeetCode #496 - Next Greater Element I
use std::collections::HashMap;

fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut st: Vec<i32> = vec![];
    let mut next: HashMap<i32, i32> = HashMap::new();
    for x in nums2 {
        while let Some(&t) = st.last() {
            if t < x {
                st.pop();
                next.insert(t, x);
            } else {
                break;
            }
        }
        st.push(x);
    }
    nums1.into_iter().map(|x| *next.get(&x).unwrap_or(&-1)).collect()
}

fn main() {
    println!("{:?}", next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::next_greater_element;

    #[test]
    fn example_one() {
        assert_eq!(next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]), vec![-1, 3, -1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(next_greater_element(vec![2, 4], vec![1, 2, 3, 4]), vec![3, -1]);
    }
}
