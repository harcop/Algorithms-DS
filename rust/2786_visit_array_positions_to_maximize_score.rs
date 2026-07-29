/// LeetCode #2786 - Visit Array Positions to Maximize Score
fn max_score(nums: Vec<i32>, x: i32) -> i64 {
    const INF: i64 = 1 << 60;
    let mut f = [-(INF); 2];
    f[(nums[0] & 1) as usize] = nums[0] as i64;
    for &v in &nums[1..] {
        let p = (v & 1) as usize;
        let other = p ^ 1;
        f[p] = f[p].max(f[other] - x as i64) + v as i64;
    }
    f[0].max(f[1])
}

fn main() {
    println!("{}", max_score(vec![2, 3, 6, 1, 9, 2], 5));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example_one() {
        assert_eq!(max_score(vec![2, 3, 6, 1, 9, 2], 5), 13);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score(vec![2, 4, 6, 8], 3), 20);
    }
}
