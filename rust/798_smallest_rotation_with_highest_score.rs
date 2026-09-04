/// LeetCode #798 - Smallest Rotation with Highest Score
fn best_rotation(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut change = vec![0i32; n];
    for (i, &a) in nums.iter().enumerate() {
        change[(i + n - a as usize + 1) % n] -= 1;
        change[(i + 1) % n] += 1;
    }
    let mut score: i32 = nums
        .iter()
        .enumerate()
        .map(|(i, &a)| if a <= i as i32 { 1 } else { 0 })
        .sum();
    let mut best = score;
    let mut ans = 0i32;
    for k in 1..n {
        score += change[k];
        if score > best {
            best = score;
            ans = k as i32;
        }
    }
    ans
}

fn main() {
    println!("{}", best_rotation(vec![2, 3, 1, 4, 0]));
}

#[cfg(test)]
mod tests {
    use super::best_rotation;

    #[test]
    fn example_one() {
        assert_eq!(best_rotation(vec![2, 3, 1, 4, 0]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(best_rotation(vec![1, 3, 0, 2, 4]), 0);
    }
}
