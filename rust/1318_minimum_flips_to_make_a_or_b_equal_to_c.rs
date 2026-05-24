/// LeetCode #1318 - Minimum Flips to Make a OR b Equal to c
fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let mut ans = 0;
    for i in 0..32 {
        let ab = (a >> i) & 1;
        let bb = (b >> i) & 1;
        let cb = (c >> i) & 1;
        if cb == 1 {
            if ab == 0 && bb == 0 {
                ans += 1;
            }
        } else if ab == 1 || bb == 1 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", min_flips(2, 6, 5));
}

#[cfg(test)]
mod tests {
    use super::min_flips;

    #[test]
    fn example_one() {
        assert_eq!(min_flips(2, 6, 5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_flips(4, 2, 7), 1);
    }
}
