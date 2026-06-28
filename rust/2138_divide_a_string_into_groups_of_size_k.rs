/// LeetCode #2138 - Divide a String Into Groups of Size k
fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let k = k as usize;
    let mut chars: Vec<char> = s.chars().collect();
    while chars.len() % k != 0 {
        chars.push(fill);
    }

    chars
        .chunks(k)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn main() {
    println!("{:?}", divide_string("abcdefghi".into(), 3, 'x'));
}

#[cfg(test)]
mod tests {
    use super::divide_string;

    #[test]
    fn example_one() {
        assert_eq!(
            divide_string("abcdefghi".into(), 3, 'x'),
            vec!["abc", "def", "ghi"]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            divide_string("abcdefghij".into(), 3, 'x'),
            vec!["abc", "def", "ghi", "jxx"]
        );
    }
}
