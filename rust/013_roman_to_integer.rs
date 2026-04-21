use std::collections::HashMap;

/// LeetCode #13 - Roman to Integer
fn roman_to_int(s: String) -> i32 {
    let map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let chars: Vec<char> = s.chars().collect();
    let mut total = 0;

    for i in 0..chars.len() {
        let current = map[&chars[i]];
        let next = if i + 1 < chars.len() { map[&chars[i + 1]] } else { 0 };
        if current < next {
            total -= current;
        } else {
            total += current;
        }
    }

    total
}

fn main() {
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}

#[cfg(test)]
mod tests {
    use super::roman_to_int;

    #[test]
    fn example_one() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn example_three() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
