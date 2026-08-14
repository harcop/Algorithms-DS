/// LeetCode #3207 - Maximum Points After Enemy Battles
fn maximum_points(mut enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
    enemy_energies.sort_unstable();
    if current_energy < enemy_energies[0] {
        return 0;
    }
    let mut ans = 0i64;
    let mut cur = current_energy as i64;
    let mn = enemy_energies[0] as i64;
    for i in (0..enemy_energies.len()).rev() {
        ans += cur / mn;
        cur %= mn;
        cur += enemy_energies[i] as i64;
    }
    ans
}

fn main() {
    println!("{}", maximum_points(vec![3, 2, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_points;

    #[test]
    fn example1() {
        assert_eq!(maximum_points(vec![3, 2, 2], 2), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_points(vec![2], 10), 5);
    }
}
