/// LeetCode #3932 - Count K-th Roots in a Range
fn count_kth_roots(l: i32, r: i32, k: i32) -> i32 {
    if k == 1 {
        return r - l + 1;
    }
    let l = l as i64;
    let r = r as i64;
    let mut ans = 0i32;
    let mut x = 0i64;
    loop {
        let mut y = 1i64;
        let mut over = false;
        for _ in 0..k {
            if x != 0 && y > r / x {
                over = true;
                break;
            }
            y *= x;
        }
        if over || y > r {
            break;
        }
        if y >= l {
            ans += 1;
        }
        x += 1;
    }
    ans
}

fn main() {
    println!("{}", count_kth_roots(1, 9, 3));
}

#[cfg(test)]
mod tests {
    use super::count_kth_roots;

    #[test]
    fn example1() {
        assert_eq!(count_kth_roots(1, 9, 3), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_kth_roots(8, 30, 2), 3);
    }
}
