/// LeetCode #4008 - Minimum Initial Strength to Defeat All Monsters
fn min_initial_strength(monsters: Vec<i32>, boosts: Vec<Vec<i32>>) -> i64 {
    let n = monsters.len();
    let mut diff = vec![0i64; n + 1];
    for b in boosts {
        diff[b[0] as usize] += b[2] as i64;
        diff[b[1] as usize + 1] -= b[2] as i64;
    }
    let check = |mut v: i64| -> bool {
        let mut bonus = 0i64;
        for i in 0..n {
            bonus += diff[i];
            if v + bonus < monsters[i] as i64 {
                return false;
            }
            v -= monsters[i] as i64;
            if v < 0 {
                v = 0;
            }
        }
        true
    };
    let mut left = 0i64;
    let mut right = 1_000_000_000_000_000i64;
    while left < right {
        let mid = (left + right) / 2;
        if check(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    println!(
        "{}",
        min_initial_strength(vec![5, 10, 15], vec![vec![1, 1, 10]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_initial_strength;

    #[test]
    fn example1() {
        assert_eq!(
            min_initial_strength(vec![5, 10, 15], vec![vec![1, 1, 10]]),
            30
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_initial_strength(vec![5, 10, 15], vec![vec![1, 2, 10], vec![1, 2, 5]]),
            5
        );
    }
}
