/// LeetCode #1319 - Maximum Number of Balloons
fn max_number_of_balloons(text: String) -> i32 {
    let mut cnt = [0i32; 26];
    for c in text.chars() {
        cnt[(c as u8 - b'a') as usize] += 1;
    }
    cnt[(b'b' - b'a') as usize]
        .min(cnt[(b'a' - b'a') as usize])
        .min(cnt[(b'l' - b'a') as usize] / 2)
        .min(cnt[(b'o' - b'a') as usize] / 2)
        .min(cnt[(b'n' - b'a') as usize])
}

fn main() {
    println!("{}", max_number_of_balloons("nlaebolko".to_string()));
}

#[cfg(test)]
mod tests {
    use super::max_number_of_balloons;

    #[test]
    fn example_one() {
        assert_eq!(max_number_of_balloons("nlaebolko".to_string()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_number_of_balloons("loonbalxballpoon".to_string()), 2);
    }
}
