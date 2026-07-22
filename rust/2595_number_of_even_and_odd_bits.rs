/// LeetCode #2595 - Number of Even and Odd Bits
fn even_odd_bit(mut n: i32) -> Vec<i32> {
    let mut ans = vec![0, 0];
    let mut i = 0usize;
    while n != 0 {
        ans[i] += n & 1;
        n >>= 1;
        i ^= 1;
    }
    ans
}

fn main() {
    println!("{:?}", even_odd_bit(50));
}

#[cfg(test)]
mod tests {
    use super::even_odd_bit;

    #[test]
    fn example_one() {
        assert_eq!(even_odd_bit(50), vec![1, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(even_odd_bit(2), vec![0, 1]);
    }
}
