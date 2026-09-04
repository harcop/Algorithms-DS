/// LeetCode #1165 - Single-Row Keyboard
fn calculate_time(keyboard: String, word: String) -> i32 {
    let mut pos = [0i32; 26];
    for (i, c) in keyboard.chars().enumerate() {
        pos[(c as u8 - b'a') as usize] = i as i32;
    }
    let mut cur = 0i32;
    let mut time = 0i32;
    for c in word.chars() {
        let p = pos[(c as u8 - b'a') as usize];
        time += (p - cur).abs();
        cur = p;
    }
    time
}

fn main() {
    println!(
        "{}",
        calculate_time("abcdefghijklmnopqrstuvwxyz".into(), "cba".into())
    );
}

#[cfg(test)]
mod tests {
    use super::calculate_time;

    #[test]
    fn example_one() {
        assert_eq!(
            calculate_time("abcdefghijklmnopqrstuvwxyz".into(), "cba".into()),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            calculate_time("pqrstuvwxyzabcdefghijklmno".into(), "leetcode".into()),
            73
        );
    }
}
