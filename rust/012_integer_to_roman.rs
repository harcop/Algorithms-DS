/// LeetCode #12 - Integer to Roman
fn int_to_roman(num: i32) -> String {
    let symbols = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut n = num;
    let mut out = String::new();

    for (value, symbol) in symbols {
        while n >= value {
            n -= value;
            out.push_str(symbol);
        }
    }

    out
}

fn main() {
    println!("{}", int_to_roman(1994));
}

#[cfg(test)]
mod tests {
    use super::int_to_roman;

    #[test]
    fn example_one() {
        assert_eq!(int_to_roman(3), "III");
    }

    #[test]
    fn example_two() {
        assert_eq!(int_to_roman(58), "LVIII");
    }

    #[test]
    fn example_three() {
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}
