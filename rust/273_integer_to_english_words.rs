/// LeetCode #273 - Integer to English Words
fn number_to_words(mut num: i32) -> String {
    if num == 0 {
        return "Zero".into();
    }
    const BELOW_20: [&str; 20] = [
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
        "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen",
        "Eighteen", "Nineteen",
    ];
    const TENS: [&str; 10] = [
        "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
    ];
    fn three(n: i32, below_20: &[&str; 20], tens: &[&str; 10]) -> String {
        let mut s = String::new();
        let mut n = n;
        if n >= 100 {
            s.push_str(below_20[(n / 100) as usize]);
            s.push_str(" Hundred ");
            n %= 100;
        }
        if n >= 20 {
            s.push_str(tens[(n / 10) as usize]);
            s.push(' ');
            n %= 10;
        }
        if n > 0 {
            s.push_str(below_20[n as usize]);
            s.push(' ');
        }
        s.trim_end().to_string()
    }
    let scales = ["", "Thousand", "Million", "Billion"];
    let mut parts = vec![];
    let mut i = 0;
    while num > 0 {
        if num % 1000 != 0 {
            let mut p = three(num % 1000, &BELOW_20, &TENS);
            if !scales[i].is_empty() {
                p.push(' ');
                p.push_str(scales[i]);
            }
            parts.push(p);
        }
        num /= 1000;
        i += 1;
    }
    parts.reverse();
    parts.join(" ").trim().to_string()
}

fn main() {
    println!("{}", number_to_words(123));
}

#[cfg(test)]
mod tests {
    use super::number_to_words;

    #[test]
    fn example_one() {
        assert_eq!(number_to_words(123), "One Hundred Twenty Three");
    }

    #[test]
    fn example_two() {
        assert_eq!(
            number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five"
        );
    }
}
