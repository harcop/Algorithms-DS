/// LeetCode #1286 - Iterator for Combination
struct CombinationIterator {
    chars: Vec<char>,
    idx: Vec<usize>,
    has: bool,
}

impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let chars: Vec<char> = characters.chars().collect();
        let k = combination_length as usize;
        let has = k > 0 && k <= chars.len();
        Self {
            chars,
            idx: (0..k).collect(),
            has,
        }
    }

    fn next(&mut self) -> String {
        let s: String = self.idx.iter().map(|&i| self.chars[i]).collect();
        self.advance();
        s
    }

    fn has_next(&self) -> bool {
        self.has
    }

    fn advance(&mut self) {
        let n = self.chars.len();
        let k = self.idx.len();
        let mut i = k.wrapping_sub(1);
        while i < k && self.idx[i] == n - k + i {
            if i == 0 {
                self.has = false;
                return;
            }
            i -= 1;
        }
        self.idx[i] += 1;
        for j in (i + 1)..k {
            self.idx[j] = self.idx[j - 1] + 1;
        }
    }
}

fn main() {
    let mut it = CombinationIterator::new("abc".to_string(), 2);
    println!("{}", it.next());
}

#[cfg(test)]
mod tests {
    use super::CombinationIterator;

    #[test]
    fn example_sequence() {
        let mut it = CombinationIterator::new("abc".to_string(), 2);
        assert!(it.has_next());
        assert_eq!(it.next(), "ab");
        assert!(it.has_next());
        assert_eq!(it.next(), "ac");
        assert!(it.has_next());
        assert_eq!(it.next(), "bc");
        assert!(!it.has_next());
    }
}
