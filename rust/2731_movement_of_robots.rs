/// LeetCode #2731 - Movement of Robots
fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let bytes = s.as_bytes();
    let mut arr: Vec<i64> = nums
        .iter()
        .enumerate()
        .map(|(i, &x)| x as i64 + if bytes[i] == b'L' { -d as i64 } else { d as i64 })
        .collect();
    arr.sort_unstable();
    let mut ans = 0i64;
    let mut sum = 0i64;
    for (i, &x) in arr.iter().enumerate() {
        ans = (ans + i as i64 * x - sum).rem_euclid(MOD);
        sum += x;
    }
    ans as i32
}

fn main() {
    println!("{}", sum_distance(vec![-2, 0, 2], "RLL".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::sum_distance;

    #[test]
    fn example_one() {
        assert_eq!(sum_distance(vec![-2, 0, 2], "RLL".into(), 3), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_distance(vec![1, 0], "RL".into(), 2), 5);
    }
}
