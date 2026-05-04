/// LeetCode #313 - Super Ugly Number
fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut ugly = vec![1i64; n];
    let mut idx: Vec<usize> = vec![0; primes.len()];
    for i in 1..n {
        let mut next = i64::MAX;
        for (j, &p) in primes.iter().enumerate() {
            next = next.min(ugly[idx[j]] * p as i64);
        }
        ugly[i] = next;
        for (j, &p) in primes.iter().enumerate() {
            if ugly[idx[j]] * p as i64 == next {
                idx[j] += 1;
            }
        }
    }
    ugly[n - 1] as i32
}

fn main() {
    println!("{}", nth_super_ugly_number(12, vec![2, 7, 13, 19]));
}

#[cfg(test)]
mod tests {
    use super::nth_super_ugly_number;

    #[test]
    fn example_one() {
        assert_eq!(nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
    }
}
