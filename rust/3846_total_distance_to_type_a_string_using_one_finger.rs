/// LeetCode #3846 - Total Distance to Type a String Using One Finger (premium)
fn total_distance(s: String) -> i32 {
    let keys = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
    let mut pos = [(0i32, 0i32); 26];
    for (i, row) in keys.iter().enumerate() {
        for (j, c) in row.bytes().enumerate() {
            pos[(c - b'a') as usize] = (i as i32, j as i32);
        }
    }
    let mut pre = b'a';
    let mut ans = 0;
    for cur in s.bytes() {
        let (x1, y1) = pos[(pre - b'a') as usize];
        let (x2, y2) = pos[(cur - b'a') as usize];
        ans += (x1 - x2).abs() + (y1 - y2).abs();
        pre = cur;
    }
    ans
}

fn main() {
    println!("{}", total_distance("hello".into()));
}

#[cfg(test)]
mod tests {
    use super::total_distance;

    #[test]
    fn example1() {
        assert_eq!(total_distance("hello".into()), 17);
    }

    #[test]
    fn example2() {
        assert_eq!(total_distance("a".into()), 0);
    }
}
