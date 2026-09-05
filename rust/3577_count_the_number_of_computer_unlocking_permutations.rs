/// LeetCode #3577 - Count the Number of Computer Unlocking Permutations
fn count_permutations(complexity: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 1i64;
    for i in 1..complexity.len() {
        if complexity[i] <= complexity[0] {
            return 0;
        }
        ans = ans * i as i64 % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", count_permutations(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::count_permutations;

    #[test]
    fn example1() {
        assert_eq!(count_permutations(vec![1, 2, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_permutations(vec![3, 3, 3, 4, 4, 4]), 0);
    }
}
