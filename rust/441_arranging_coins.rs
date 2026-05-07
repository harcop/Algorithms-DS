/// LeetCode #441 - Arranging Coins
fn arrange_coins(mut n: i64) -> i32 {
    let mut lo = 1i64;
    let mut hi = n;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let need = mid * (mid + 1) / 2;
        if need == n {
            return mid as i32;
        }
        if need < n {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    hi as i32
}

fn main() {
    println!("{}", arrange_coins(5));
}

#[cfg(test)]
mod tests {
    use super::arrange_coins;

    #[test]
    fn example_one() {
        assert_eq!(arrange_coins(5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(arrange_coins(8), 3);
    }
}
