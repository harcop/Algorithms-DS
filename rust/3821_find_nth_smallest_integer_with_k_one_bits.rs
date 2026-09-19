/// LeetCode #3821 - Find Nth Smallest Integer With K One Bits
fn nth_smallest(mut n: i64, mut k: i32) -> i64 {
    const MX: usize = 50;
    let mut c = [[0i64; MX + 1]; MX];
    for i in 0..MX {
        c[i][0] = 1;
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    let mut ans = 0i64;
    for i in (0..50).rev() {
        if n > c[i][k as usize] {
            n -= c[i][k as usize];
            ans |= 1 << i;
            k -= 1;
            if k == 0 {
                break;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", nth_smallest(4, 2));
}

#[cfg(test)]
mod tests {
    use super::nth_smallest;

    #[test]
    fn example1() {
        assert_eq!(nth_smallest(4, 2), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(nth_smallest(3, 1), 4);
    }
}
