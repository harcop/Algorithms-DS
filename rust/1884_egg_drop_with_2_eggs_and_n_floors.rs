/// LeetCode #1884 - Egg Drop With 2 Eggs and N Floors
const INF: i32 = i32::MAX / 2;

fn two_egg_drop(n: i32) -> i32 {
    let n = n as usize;
    let mut f = vec![0i32; n + 1];
    for i in 1..=n {
        f[i] = INF;
        for j in 1..=i {
            f[i] = f[i].min(1 + (j as i32 - 1).max(f[i - j]));
        }
    }
    f[n]
}

fn main() {
    println!("{}", two_egg_drop(2));
}

#[cfg(test)]
mod tests {
    use super::two_egg_drop;

    #[test]
    fn example_one() {
        assert_eq!(two_egg_drop(2), 2);
    }
}
