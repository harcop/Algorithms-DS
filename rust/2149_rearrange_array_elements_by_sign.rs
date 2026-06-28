/// LeetCode #2149 - Rearrange Array Elements by Sign
fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut positives = Vec::new();
    let mut negatives = Vec::new();

    for value in nums {
        if value > 0 {
            positives.push(value);
        } else {
            negatives.push(value);
        }
    }

    let mut answer = Vec::with_capacity(positives.len() + negatives.len());
    for i in 0..positives.len() {
        answer.push(positives[i]);
        answer.push(negatives[i]);
    }

    answer
}

fn main() {
    println!("{:?}", rearrange_array(vec![3, 1, -2, -5, 2, -4]));
}

#[cfg(test)]
mod tests {
    use super::rearrange_array;

    #[test]
    fn example_one() {
        assert_eq!(
            rearrange_array(vec![3, 1, -2, -5, 2, -4]),
            vec![3, -2, 1, -5, 2, -4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(rearrange_array(vec![-1, 1]), vec![1, -1]);
    }
}
