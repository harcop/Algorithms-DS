/// LeetCode #1220 - Count Vowels Permutation
const MOD: i64 = 1_000_000_007;

fn count_vowel_permutation(n: i32) -> i32 {
    let n = n as usize;
    let mut a = 1i64;
    let mut e = 1;
    let mut i = 1;
    let mut o = 1;
    let mut u = 1;
    for _ in 1..n {
        let na = (e + i + u) % MOD;
        let ne = (a + i) % MOD;
        let ni = (e + o) % MOD;
        let no = i % MOD;
        let nu = (i + o) % MOD;
        a = na;
        e = ne;
        i = ni;
        o = no;
        u = nu;
    }
    ((a + e + i + o + u) % MOD) as i32
}

fn main() {
    println!("{}", count_vowel_permutation(1));
}

#[cfg(test)]
mod tests {
    use super::count_vowel_permutation;

    #[test]
    fn example_one() {
        assert_eq!(count_vowel_permutation(1), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_vowel_permutation(2), 10);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_vowel_permutation(5), 68);
    }
}
