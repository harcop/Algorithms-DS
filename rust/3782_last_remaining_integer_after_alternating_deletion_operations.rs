/// LeetCode #3782 - Last Remaining Integer After Alternating Deletion Operations
fn last_integer(mut n: i64) -> i64 {
    let mut result = 1i64;
    let mut l = 1i64;
    let mut from_right = false;
    while n != 1 {
        if from_right && n % 2 == 0 {
            result += l;
        }
        n = (n + 1) / 2;
        l <<= 1;
        from_right = !from_right;
    }
    result
}

fn main() {
    println!("{}", last_integer(8));
}

#[cfg(test)]
mod tests {
    use super::last_integer;

    #[test]
    fn example1() {
        assert_eq!(last_integer(8), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(last_integer(5), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(last_integer(1), 1);
    }
}
