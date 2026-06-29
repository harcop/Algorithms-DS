/// LeetCode #2155 - All Divisions With the Highest Score of a Binary Array
fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
    let mut left_zeros = 0i32;
    let mut right_ones = nums.iter().filter(|&&x| x == 1).count() as i32;
    let mut best = -1i32;
    let mut ans = Vec::new();

    for i in 0..=nums.len() {
        let score = left_zeros + right_ones;
        if score > best {
            best = score;
            ans.clear();
            ans.push(i as i32);
        } else if score == best {
            ans.push(i as i32);
        }

        if i < nums.len() {
            if nums[i] == 0 {
                left_zeros += 1;
            } else {
                right_ones -= 1;
            }
        }
    }

    ans
}

fn main() {
    println!("{:?}", max_score_indices(vec![0, 0, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::max_score_indices;

    #[test]
    fn example_one() {
        assert_eq!(max_score_indices(vec![0, 0, 1, 0]), vec![2, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score_indices(vec![0, 0, 0]), vec![3]);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_score_indices(vec![1, 1]), vec![0]);
    }
}
