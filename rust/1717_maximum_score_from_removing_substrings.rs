/// LeetCode #1717 - Maximum Score From Removing Substrings
fn maximum_gain(s: String, mut x: i32, mut y: i32) -> i32 {
    let (mut a, mut b) = (b'a', b'b');
    if x < y {
        std::mem::swap(&mut x, &mut y);
        std::mem::swap(&mut a, &mut b);
    }
    let mut ans = 0i32;
    let mut cnt1 = 0i32;
    let mut cnt2 = 0i32;
    for c in s.bytes() {
        if c == a {
            cnt1 += 1;
        } else if c == b {
            if cnt1 > 0 {
                ans += x;
                cnt1 -= 1;
            } else {
                cnt2 += 1;
            }
        } else {
            ans += cnt1.min(cnt2) * y;
            cnt1 = 0;
            cnt2 = 0;
        }
    }
    ans + cnt1.min(cnt2) * y
}
fn main() {
    println!("{}", maximum_gain("cdbcbbaaabab".into(), 4, 5));
}
#[cfg(test)]
mod tests {
    use super::maximum_gain;
    #[test]
    fn example_one() {
        assert_eq!(maximum_gain("cdbcbbaaabab".into(), 4, 5), 19);
    }
    #[test]
    fn example_two() {
        assert_eq!(maximum_gain("aabbaaxybbaabb".into(), 5, 4), 20);
    }
}
