/// LeetCode #3445 - Maximum Difference Between Even and Odd Frequency II
fn max_difference(s: String, k: i32) -> i32 {
    let s: Vec<i32> = s.bytes().map(|c| (c - b'0') as i32).collect();
    let k = k as i32;
    const INF: i32 = i32::MAX / 4;
    let mut ans = -INF;
    for a in 0..5 {
        for b in 0..5 {
            if a == b {
                continue;
            }
            let mut cur_a = 0;
            let mut cur_b = 0;
            let mut pre_a = 0;
            let mut pre_b = 0;
            let mut t = [[INF; 2]; 2];
            let mut l = -1i32;
            for (r, &x) in s.iter().enumerate() {
                let r = r as i32;
                cur_a += (x == a) as i32;
                cur_b += (x == b) as i32;
                while r - l >= k && cur_b - pre_b >= 2 {
                    t[(pre_a & 1) as usize][(pre_b & 1) as usize] =
                        t[(pre_a & 1) as usize][(pre_b & 1) as usize].min(pre_a - pre_b);
                    l += 1;
                    pre_a += (s[l as usize] == a) as i32;
                    pre_b += (s[l as usize] == b) as i32;
                }
                ans = ans.max(
                    cur_a - cur_b - t[(cur_a & 1 ^ 1) as usize][(cur_b & 1) as usize],
                );
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_difference("12233".into(), 4));
}

#[cfg(test)]
mod tests {
    use super::max_difference;

    #[test]
    fn example1() {
        assert_eq!(max_difference("12233".into(), 4), -1);
    }

    #[test]
    fn example2() {
        assert_eq!(max_difference("1122211".into(), 3), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(max_difference("110".into(), 3), -1);
    }
}
