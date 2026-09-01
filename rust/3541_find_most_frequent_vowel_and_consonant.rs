/// LeetCode #3541 - Find Most Frequent Vowel and Consonant
fn max_freq_sum(s: String) -> i32 {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let mut a = 0;
    let mut b = 0;
    for (i, &v) in cnt.iter().enumerate() {
        let c = (b'a' + i as u8) as char;
        if "aeiou".contains(c) {
            a = a.max(v);
        } else {
            b = b.max(v);
        }
    }
    a + b
}

fn main() {
    println!("{}", max_freq_sum("successes".into()));
}

#[cfg(test)]
mod tests {
    use super::max_freq_sum;

    #[test]
    fn example1() {
        assert_eq!(max_freq_sum("successes".into()), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(max_freq_sum("aeiaeia".into()), 3);
    }
}
