/// LeetCode #3079 - Find the Sum of Encrypted Integers
fn encrypt(mut x: i32) -> i32 {
    let mut digits = Vec::new();
    if x == 0 {
        return 0;
    }
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    let max_d = *digits.iter().max().unwrap();
    let mut res = 0;
    for _ in 0..digits.len() {
        res = res * 10 + max_d;
    }
    res
}

fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    nums.into_iter().map(encrypt).sum()
}

fn main() {
    println!("{}", sum_of_encrypted_int(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::sum_of_encrypted_int;

    #[test]
    fn example1() {
        assert_eq!(sum_of_encrypted_int(vec![1, 2, 3]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_encrypted_int(vec![10, 21, 31]), 66);
    }
}
