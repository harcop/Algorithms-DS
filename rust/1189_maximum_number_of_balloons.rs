/// LeetCode #1189 - Maximum Number of Balloons
fn max_number_of_balloons(text: String) -> i32 {
    let mut cnt = [0i32; 26];
    for c in text.bytes() {
        cnt[(c - b'a') as usize] += 1;
    }
    let b = cnt[1];
    let a = cnt[0];
    let l = cnt[11] / 2;
    let o = cnt[14] / 2;
    let n = cnt[13];
    b.min(a).min(l).min(o).min(n)
}

fn main() {
    println!("{}", max_number_of_balloons("nlaebolko".into()));
}

#[cfg(test)]
mod tests {
    use super::max_number_of_balloons;

    #[test]
    fn example_one() {
        assert_eq!(max_number_of_balloons("nlaebolko".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_number_of_balloons("loonbalxballpoon".into()), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_number_of_balloons("leetcode".into()), 0);
    }
}
