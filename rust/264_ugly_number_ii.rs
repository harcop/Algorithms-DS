/// LeetCode #264 - Ugly Number II
fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut ugly = vec![1i64; n];
    let mut i2 = 0usize;
    let mut i3 = 0usize;
    let mut i5 = 0usize;
    for i in 1..n {
        let next = (ugly[i2] * 2).min(ugly[i3] * 3).min(ugly[i5] * 5);
        ugly[i] = next;
        if ugly[i2] * 2 == next {
            i2 += 1;
        }
        if ugly[i3] * 3 == next {
            i3 += 1;
        }
        if ugly[i5] * 5 == next {
            i5 += 1;
        }
    }
    ugly[n - 1] as i32
}

fn main() {
    println!("{}", nth_ugly_number(10));
}

#[cfg(test)]
mod tests {
    use super::nth_ugly_number;

    #[test]
    fn example_one() {
        assert_eq!(nth_ugly_number(10), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(nth_ugly_number(1), 1);
    }
}
