/// LeetCode #3344 - Maximum Sized Array
fn max_sized_array(s: i64) -> i32 {
    const MX: usize = 1330;
    let mut f = vec![0i64; MX];
    for i in 1..MX {
        f[i] = f[i - 1] + i as i64;
        for j in 0..i {
            f[i] += 2 * (i as i64 | j as i64);
        }
    }
    let mut l = 1usize;
    let mut r = MX;
    while l < r {
        let m = (l + r + 1) >> 1;
        let prod = f[m - 1] as i128 * (m as i128 - 1) * m as i128 / 2;
        if prod <= s as i128 {
            l = m;
        } else {
            r = m - 1;
        }
    }
    l as i32
}

fn main() {
    println!("{}", max_sized_array(10));
}

#[cfg(test)]
mod tests {
    use super::max_sized_array;

    #[test]
    fn example1() {
        assert_eq!(max_sized_array(10), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_sized_array(0), 1);
    }
}
