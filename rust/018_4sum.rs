/// LeetCode #18 - 4Sum
fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let n = nums.len();
    let mut result = Vec::new();

    if n < 4 {
        return result;
    }

    for i in 0..(n - 3) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in (i + 1)..(n - 2) {
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }

            let mut left = j + 1;
            let mut right = n - 1;

            while left < right {
                let sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;
                let target64 = target as i64;

                if sum < target64 {
                    left += 1;
                } else if sum > target64 {
                    right -= 1;
                } else {
                    result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
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
    }

    result
}

fn main() {
    println!("{:?}", four_sum(vec![1, 0, -1, 0, -2, 2], 0));
}

#[cfg(test)]
mod tests {
    use super::four_sum;

    fn normalize(mut quads: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for q in &mut quads {
            q.sort_unstable();
        }
        quads.sort_unstable();
        quads
    }

    #[test]
    fn example_one() {
        let got = normalize(four_sum(vec![1, 0, -1, 0, -2, 2], 0));
        let expected = normalize(vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(four_sum(vec![2, 2, 2, 2, 2], 8), vec![vec![2, 2, 2, 2]]);
    }
}
