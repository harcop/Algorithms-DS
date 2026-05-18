/// LeetCode #1055 - Shortest Way to Form String
fn shortest_way(source: String, target: String) -> i32 {
    let src: Vec<char> = source.chars().collect();
    let mut next = vec![vec![src.len(); 26]; src.len() + 1];
    for i in (0..src.len()).rev() {
        next[i] = next[i + 1].clone();
        next[i][src[i] as usize - b'a' as usize] = i;
    }
    let mut subs = 0i32;
    let mut idx = 0usize;
    for c in target.chars() {
        let ci = c as usize - b'a' as usize;
        if next[idx][ci] == src.len() {
            return -1;
        }
        let ni = next[idx][ci] + 1;
        if ni == src.len() {
            subs += 1;
            idx = 0;
        } else {
            idx = ni;
        }
    }
    subs
}

fn main() {
    println!("{}", shortest_way("abc".into(), "abcbc".into()));
}

#[cfg(test)]
mod tests {
    use super::shortest_way;

    #[test]
    fn example_one() {
        assert_eq!(shortest_way("abc".into(), "abcbc".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(shortest_way("abc".into(), "acdbc".into()), -1);
    }
}
