/// LeetCode #443 - String Compression
fn compress(chars: &mut Vec<char>) -> i32 {
    let mut write = 0usize;
    let n = chars.len();
    let mut i = 0usize;
    while i < n {
        let ch = chars[i];
        let mut j = i + 1;
        while j < n && chars[j] == ch {
            j += 1;
        }
        chars[write] = ch;
        write += 1;
        let len = j - i;
        if len > 1 {
            let s = len.to_string();
            for c in s.chars() {
                chars[write] = c;
                write += 1;
            }
        }
        i = j;
    }
    chars.truncate(write);
    write as i32
}

fn main() {
    let mut v = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    println!("{}", compress(&mut v));
}

#[cfg(test)]
mod tests {
    use super::compress;

    #[test]
    fn example_one() {
        let mut v = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(compress(&mut v), 6);
        assert_eq!(v, vec!['a', '2', 'b', '2', 'c', '3']);
    }
}
