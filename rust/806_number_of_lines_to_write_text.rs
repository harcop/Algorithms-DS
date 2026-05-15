/// LeetCode #806 - Number of Lines To Write Text
fn number_of_lines(heights: Vec<i32>, s: String) -> Vec<i32> {
    let mut lines = 1i32;
    let mut width = 0i32;
    for c in s.bytes() {
        let h = heights[(c - b'a') as usize];
        if width + h > 100 {
            lines += 1;
            width = h;
        } else {
            width += h;
        }
    }
    vec![lines, width]
}

fn main() {
    let h: Vec<i32> = (0..26).map(|i| 10 + (i % 5) as i32).collect();
    println!("{:?}", number_of_lines(h, "abcdefghijklmnopqrstuvwxyz".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_lines;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_lines(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], "abcdefghijklmnopqrstuvwxyz".into()),
            vec![3, 60]
        );
    }
}
