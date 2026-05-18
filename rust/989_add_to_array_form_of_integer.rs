/// LeetCode #989 - Add to Array-Form of Integer
fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
    let mut carry = k as i64;
    let mut out = Vec::new();
    for &d in num.iter().rev() {
        carry += d as i64;
        out.push((carry % 10) as i32);
        carry /= 10;
    }
    while carry > 0 {
        out.push((carry % 10) as i32);
        carry /= 10;
    }
    out.reverse();
    out
}

fn main() {
    println!("{:?}", add_to_array_form(vec![1, 2, 0, 0], 34));
}

#[cfg(test)]
mod tests {
    use super::add_to_array_form;

    #[test]
    fn example_one() {
        assert_eq!(add_to_array_form(vec![1, 2, 0, 0], 34), vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(add_to_array_form(vec![2, 7, 4], 181), vec![4, 5, 5]);
    }
}
