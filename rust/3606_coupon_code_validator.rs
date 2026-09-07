/// LeetCode #3606 - Coupon Code Validator
use std::collections::HashSet;

fn check(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
    let bs: HashSet<&str> = ["electronics", "grocery", "pharmacy", "restaurant"]
        .iter()
        .copied()
        .collect();
    let mut idx = Vec::new();
    for i in 0..code.len() {
        if is_active[i] && bs.contains(business_line[i].as_str()) && check(&code[i]) {
            idx.push(i);
        }
    }
    idx.sort_by(|&i, &j| {
        business_line[i]
            .cmp(&business_line[j])
            .then(code[i].cmp(&code[j]))
    });
    idx.into_iter().map(|i| code[i].clone()).collect()
}

fn main() {
    println!("{:?}", validate_coupons(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::validate_coupons;

    fn ss(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn example1() {
        assert_eq!(
            validate_coupons(
                ss(&["SAVE20", "", "PHARMA5", "SAVE@20"]),
                ss(&["restaurant", "grocery", "pharmacy", "restaurant"]),
                vec![true, true, true, true]
            ),
            vec!["PHARMA5", "SAVE20"]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            validate_coupons(
                ss(&["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"]),
                ss(&["grocery", "electronics", "invalid"]),
                vec![false, true, true]
            ),
            vec!["ELECTRONICS_50"]
        );
    }
}
