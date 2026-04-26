/// LeetCode #43 - Multiply Strings
fn multiply(num1: String, num2: String) -> String {
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }

    let n1 = num1.as_bytes();
    let n2 = num2.as_bytes();
    let mut res = vec![0; n1.len() + n2.len()];

    for i in (0..n1.len()).rev() {
        for j in (0..n2.len()).rev() {
            let mul = (n1[i] - b'0') as i32 * (n2[j] - b'0') as i32;
            let p1 = i + j;
            let p2 = i + j + 1;
            let sum = mul + res[p2];
            res[p2] = sum % 10;
            res[p1] += sum / 10;
        }
    }

    let mut out = String::new();
    let mut started = false;
    for digit in res {
        if digit != 0 || started {
            started = true;
            out.push((digit as u8 + b'0') as char);
        }
    }

    if out.is_empty() {
        "0".to_string()
    } else {
        out
    }
}

fn main() {
    println!("{}", multiply("123".to_string(), "456".to_string()));
}

#[cfg(test)]
mod tests {
    use super::multiply;

    #[test]
    fn example_one() {
        assert_eq!(multiply("2".to_string(), "3".to_string()), "6");
    }

    #[test]
    fn example_two() {
        assert_eq!(multiply("123".to_string(), "456".to_string()), "56088");
    }
}
