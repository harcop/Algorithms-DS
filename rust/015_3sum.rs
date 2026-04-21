/// LeetCode #15 - 3Sum
///
/// Sort + fixed element + two pointers, skipping duplicates.
fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let n = nums.len();
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..n {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        if nums[i] > 0 {
            break;
        }

        let mut left = i + 1;
        let mut right = n.saturating_sub(1);

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum < 0 {
                left += 1;
            } else if sum > 0 {
                right -= 1;
            } else {
                result.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;
                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            }
        }
    }

    result
}

fn main() {
    println!("{:?}", three_sum(vec![-1, 0, 1, 2, -1, -4]));
}

#[cfg(test)]
mod tests {
    use super::three_sum;

    fn normalize(mut triples: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for triple in &mut triples {
            triple.sort_unstable();
        }
        triples.sort_unstable();
        triples
    }

    #[test]
    fn example_one() {
        let got = normalize(three_sum(vec![-1, 0, 1, 2, -1, -4]));
        let expected = normalize(vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        let got = three_sum(vec![0, 1, 1]);
        assert!(got.is_empty());
    }

    #[test]
    fn example_three() {
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
