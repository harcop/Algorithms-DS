/// LeetCode #2954 - Count the Number of Infection Sequences
const MOD: i64 = 1_000_000_007;

fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    let mut res = 1i64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    res
}

fn number_of_sequence(n: i32, sick: Vec<i32>) -> i32 {
    let mut nums = Vec::new();
    let mut prev = -1;
    for &x in &sick {
        nums.push(x - prev - 1);
        prev = x;
    }
    nums.push(n - prev - 1);

    let s: i32 = nums.iter().sum();
    let mut fac = vec![1i64; (s as usize) + 1];
    for i in 2..=s as usize {
        fac[i] = fac[i - 1] * i as i64 % MOD;
    }

    let mut ans = fac[s as usize];
    for &x in &nums {
        if x > 0 {
            ans = ans * mod_pow(fac[x as usize], MOD - 2) % MOD;
        }
    }
    for &x in &nums[1..nums.len() - 1] {
        if x > 1 {
            ans = ans * mod_pow(2, (x - 1) as i64) % MOD;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", number_of_sequence(5, vec![0, 4]));
}

#[cfg(test)]
mod tests {
    use super::number_of_sequence;

    #[test]
    fn example_one() {
        assert_eq!(number_of_sequence(5, vec![0, 4]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_sequence(4, vec![1]), 3);
    }
}
