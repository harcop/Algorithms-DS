/// LeetCode #679 - 24 Game
fn judge_point24(cards: Vec<i32>) -> bool {
    let nums: Vec<f64> = cards.iter().map(|&x| x as f64).collect();
    solve(&nums)
}

fn solve(nums: &[f64]) -> bool {
    if nums.len() == 1 {
        return (nums[0] - 24.0).abs() < 1e-6;
    }
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let rest: Vec<f64> = nums
                .iter()
                .enumerate()
                .filter(|(k, _)| *k != i && *k != j)
                .map(|(_, v)| *v)
                .collect();
            let mut candidates = vec![
                nums[i] + nums[j],
                nums[i] - nums[j],
                nums[i] * nums[j],
            ];
            if nums[j].abs() > 1e-6 {
                candidates.push(nums[i] / nums[j]);
            }
            for c in candidates {
                let mut next = rest.clone();
                next.push(c);
                if solve(&next) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    println!("{}", judge_point24(vec![4, 1, 8, 7]));
}

#[cfg(test)]
mod tests {
    use super::judge_point24;

    #[test]
    fn example_one() {
        assert!(judge_point24(vec![4, 1, 8, 7]));
    }

    #[test]
    fn example_two() {
        assert!(!judge_point24(vec![1, 2, 1, 2]));
    }
}
