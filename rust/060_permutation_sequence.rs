/// LeetCode #60 - Permutation Sequence
fn get_permutation(n: i32, k: i32) -> String {
    let n = n as usize;
    let mut nums: Vec<usize> = (1..=n).collect();
    let mut fact = vec![1usize; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i;
    }

    let mut k = (k - 1) as usize;
    let mut out = String::new();
    for i in (1..=n).rev() {
        let idx = k / fact[i - 1];
        k %= fact[i - 1];
        out.push(char::from(b'0' + nums.remove(idx) as u8));
    }
    out
}

fn main() {
    println!("{}", get_permutation(3, 3));
}

#[cfg(test)]
mod tests {
    use super::get_permutation;
    #[test]
    fn example_one() {
        assert_eq!(get_permutation(3, 3), "213");
    }
    #[test]
    fn example_two() {
        assert_eq!(get_permutation(4, 9), "2314");
    }
}
