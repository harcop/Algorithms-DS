/// LeetCode #3494 - Find the Minimum Amount of Time to Brew Potions
fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
    let n = skill.len();
    let mut f = vec![0i64; n];
    for &x in &mana {
        let x = x as i64;
        let mut tot = 0i64;
        for i in 0..n {
            tot = tot.max(f[i]) + skill[i] as i64 * x;
        }
        f[n - 1] = tot;
        for i in (0..n - 1).rev() {
            f[i] = f[i + 1] - skill[i + 1] as i64 * x;
        }
    }
    f[n - 1]
}

fn main() {
    println!("{}", min_time(vec![1, 5, 2, 4], vec![5, 1, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_time;

    #[test]
    fn example1() {
        assert_eq!(min_time(vec![1, 5, 2, 4], vec![5, 1, 4, 2]), 110);
    }

    #[test]
    fn example2() {
        assert_eq!(min_time(vec![1, 1, 1], vec![1, 1, 1]), 5);
    }

    #[test]
    fn example3() {
        assert_eq!(min_time(vec![1, 2, 3, 4], vec![1, 2]), 21);
    }
}
