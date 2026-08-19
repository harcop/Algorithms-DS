/// LeetCode #3304 - Find the K-th Character in String Game I
fn kth_character(k: i32) -> char {
    let mut word = vec![0u8];
    while word.len() < k as usize {
        let m = word.len();
        for i in 0..m {
            word.push((word[i] + 1) % 26);
        }
    }
    (b'a' + word[(k - 1) as usize]) as char
}

fn main() {
    println!("{}", kth_character(5));
}

#[cfg(test)]
mod tests {
    use super::kth_character;

    #[test]
    fn example1() {
        assert_eq!(kth_character(5), 'b');
    }

    #[test]
    fn example2() {
        assert_eq!(kth_character(10), 'c');
    }
}
