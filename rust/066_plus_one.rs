/// LeetCode #66 - Plus One
fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    for d in digits.iter_mut().rev() {
        let sum = *d + carry;
        *d = sum % 10;
        carry = sum / 10;
        if carry == 0 {
            break;
        }
    }
    if carry > 0 {
        digits.insert(0, carry);
    }
    digits
}

fn main() {
    println!("{:?}", plus_one(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::plus_one;

    #[test]
    fn example_one() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn example_three() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }
}
