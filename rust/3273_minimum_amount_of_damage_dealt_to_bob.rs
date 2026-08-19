/// LeetCode #3273 - Minimum Amount of Damage Dealt to Bob
fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
    let mut enemies: Vec<(i32, i32)> = damage
        .iter()
        .zip(health.iter())
        .map(|(&d, &h)| (d, (h + power - 1) / power))
        .collect();
    enemies.sort_by(|a, b| {
        (b.0 as i64 * a.1 as i64).cmp(&(a.0 as i64 * b.1 as i64))
    });
    let mut sum_damage: i64 = damage.iter().map(|&x| x as i64).sum();
    let mut ans = 0i64;
    for (d, t) in enemies {
        ans += sum_damage * t as i64;
        sum_damage -= d as i64;
    }
    ans
}

fn main() {
    println!("{}", min_damage(4, vec![1, 2, 3, 4], vec![4, 5, 6, 8]));
}

#[cfg(test)]
mod tests {
    use super::min_damage;

    #[test]
    fn example1() {
        assert_eq!(min_damage(4, vec![1, 2, 3, 4], vec![4, 5, 6, 8]), 39);
    }

    #[test]
    fn example2() {
        assert_eq!(min_damage(1, vec![1, 1, 1, 1], vec![1, 2, 3, 4]), 20);
    }

    #[test]
    fn example3() {
        assert_eq!(min_damage(8, vec![40], vec![59]), 320);
    }
}
