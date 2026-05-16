/// LeetCode #821 - Shortest Distance to a Character
fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let b = s.as_bytes();
    let t = c as u8;
    let n = b.len();
    let mut left = vec![i32::MAX; n];
    let mut last = -1i32;
    for i in 0..n {
        if b[i] == t {
            last = i as i32;
        }
        if last >= 0 {
            left[i] = i as i32 - last;
        }
    }
    let mut ans = vec![0i32; n];
    last = -1;
    for i in (0..n).rev() {
        if b[i] == t {
            last = i as i32;
        }
        let right = if last < 0 {
            i32::MAX
        } else {
            last - i as i32
        };
        ans[i] = left[i].min(right);
    }
    ans
}

fn main() {
    println!("{:?}", shortest_to_char("loveleetcode".into(), 'e'));
}

#[cfg(test)]
mod tests {
    use super::shortest_to_char;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_to_char("loveleetcode".into(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 1, 2, 2, 1, 0, 1, 2]
        );
    }
}
