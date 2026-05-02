/// LeetCode #171 - Excel Sheet Column Number
fn title_to_number(column_title: String) -> i32 {
    let mut n = 0i32;
    for c in column_title.bytes() {
        n = n * 26 + (c - b'A' + 1) as i32;
    }
    n
}

fn main() {
    println!("{}", title_to_number("AB".into()));
}

#[cfg(test)]
mod tests {
    use super::title_to_number;

    #[test]
    fn example_one() {
        assert_eq!(title_to_number("A".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(title_to_number("AB".into()), 28);
    }

    #[test]
    fn example_three() {
        assert_eq!(title_to_number("ZY".into()), 701);
    }
}
