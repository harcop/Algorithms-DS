/// LeetCode #3335 - Total Characters in String After Transformations I
fn length_after_transformations(s: String, t: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let mut cnt = [0i32; 26];
    for c in s.bytes() {
        cnt[(c - b'a') as usize] += 1;
    }
    for _ in 0..t {
        let mut nxt = [0i32; 26];
        nxt[0] = cnt[25];
        nxt[1] = (cnt[0] + cnt[25]) % MOD;
        for j in 2..26 {
            nxt[j] = cnt[j - 1];
        }
        cnt = nxt;
    }
    cnt.iter().fold(0, |a, &b| (a + b) % MOD)
}

fn main() {
    println!("{}", length_after_transformations("abcyy".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::length_after_transformations;

    #[test]
    fn example1() {
        assert_eq!(length_after_transformations("abcyy".into(), 2), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(length_after_transformations("azbk".into(), 1), 5);
    }
}
