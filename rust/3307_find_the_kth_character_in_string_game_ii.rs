/// LeetCode #3307 - Find the K-th Character in String Game II
fn kth_character(mut k: i64, operations: Vec<i32>) -> char {
    let mut n = 1i64;
    let mut i = 0usize;
    while n < k {
        n *= 2;
        i += 1;
    }
    let mut d = 0i64;
    while n > 1 {
        if k > n / 2 {
            k -= n / 2;
            d += operations[i - 1] as i64;
        }
        n /= 2;
        i -= 1;
    }
    (b'a' + (d % 26) as u8) as char
}

fn main() {
    println!("{}", kth_character(5, vec![0, 0, 0]));
}

#[cfg(test)]
mod tests {
    use super::kth_character;

    #[test]
    fn example1() {
        assert_eq!(kth_character(5, vec![0, 0, 0]), 'a');
    }

    #[test]
    fn example2() {
        assert_eq!(kth_character(10, vec![0, 1, 0, 1]), 'b');
    }
}
