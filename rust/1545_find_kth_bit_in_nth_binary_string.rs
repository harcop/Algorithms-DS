/// LeetCode #1545 - Find Kth Bit In Nth Binary String
fn find_kth_bit(n: i32, k: i32) -> char {
    fn rec(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }
        let len = (1 << n) - 1;
        let mid = (len + 1) / 2;
        if k == mid {
            '1'
        } else if k < mid {
            rec(n - 1, k)
        } else {
            let c = rec(n - 1, len - k + 1);
            if c == '0' { '1' } else { '0' }
        }
    }
    rec(n, k)
}

fn main() {
    println!("{}", find_kth_bit(3, 1));
}

#[cfg(test)]
mod tests {
    use super::find_kth_bit;

    #[test]
    fn example_one() {
        assert_eq!(find_kth_bit(3, 1), '0');
    }

    #[test]
    fn example_two() {
        assert_eq!(find_kth_bit(4, 11), '1');
    }
}
