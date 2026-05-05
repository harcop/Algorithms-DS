/// LeetCode #334 - Increasing Triplet Subsequence
fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut a = i32::MAX;
    let mut b = i32::MAX;
    for x in nums {
        if x <= a {
            a = x;
        } else if x <= b {
            b = x;
        } else {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", increasing_triplet(vec![1,2,3,4,5]));
}

#[cfg(test)]
mod tests {
    use super::increasing_triplet;

    #[test]
    fn example_one() {
        assert!(increasing_triplet(vec![1,2,3,4,5]));
    }

    #[test]
    fn example_two() {
        assert!(!increasing_triplet(vec![5,4,3,2,1]));
    }
}
