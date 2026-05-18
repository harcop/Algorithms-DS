/// LeetCode #1073 - Adding Two Negabinary Numbers
fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut i = arr1.len() as i32 - 1;
    let mut j = arr2.len() as i32 - 1;
    let mut carry = 0i32;
    let mut out = Vec::new();
    while i >= 0 || j >= 0 || carry != 0 {
        let a = if i >= 0 { arr1[i as usize] } else { 0 };
        let b = if j >= 0 { arr2[j as usize] } else { 0 };
        let mut sum = a + b + carry;
        let digit = sum.rem_euclid(2);
        sum -= digit;
        carry = -sum / 2;
        out.push(digit);
        i -= 1;
        j -= 1;
    }
    while out.len() > 1 && *out.last().unwrap() == 0 {
        out.pop();
    }
    out.reverse();
    out
}

fn main() {
    println!("{:?}", add_negabinary(vec![1, 1, 1, 1, 1], vec![1, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::add_negabinary;

    #[test]
    fn example_one() {
        assert_eq!(add_negabinary(vec![1, 1, 1, 1, 1], vec![1, 0, 1]), vec![1, 0, 0, 0, 0]);
    }

    #[test]
    fn example_two() {
        assert_eq!(add_negabinary(vec![0], vec![0]), vec![0]);
    }
}
