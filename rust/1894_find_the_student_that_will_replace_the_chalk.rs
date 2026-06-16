/// LeetCode #1894 - Find the Student that Will Replace the Chalk
fn chalk_replacer(chalk: Vec<i32>, mut k: i32) -> i32 {
    let s: i64 = chalk.iter().map(|&x| x as i64).sum();
    k = (k as i64 % s) as i32;
    for (i, &x) in chalk.iter().enumerate() {
        if k < x {
            return i as i32;
        }
        k -= x;
    }
    0
}

fn main() {
    println!("{}", chalk_replacer(vec![5, 1, 5], 22));
}

#[cfg(test)]
mod tests {
    use super::chalk_replacer;

    #[test]
    fn example_one() {
        assert_eq!(chalk_replacer(vec![5, 1, 5], 22), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(chalk_replacer(vec![3, 4, 1, 2], 25), 1);
    }
}
