/// LeetCode #1542 - Find Longest Awesome Substring
fn longest_awesome(s: String) -> i32 {
    let mut mask = 0i32;
    let mut first = [-1i32; 1 << 10];
    first[0] = 0;
    let mut ans = 0;
    for (i, c) in s.bytes().enumerate() {
        let bit = 1 << (c - b'0');
        mask ^= bit;
        if first[mask as usize] >= 0 {
            ans = ans.max(i as i32 + 1 - first[mask as usize]);
        } else {
            first[mask as usize] = i as i32 + 1;
        }
        for d in 0..10 {
            let m2 = mask ^ (1 << d);
            if first[m2 as usize] >= 0 {
                ans = ans.max(i as i32 + 1 - first[m2 as usize]);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", longest_awesome("3242415".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_awesome;

    #[test]
    fn example_one() {
        assert_eq!(longest_awesome("3242415".into()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_awesome("12345678".into()), 1);
    }
}
