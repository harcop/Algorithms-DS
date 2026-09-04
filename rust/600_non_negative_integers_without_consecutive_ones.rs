/// LeetCode #600 - Non-negative Integers without Consecutive Ones
fn find_integers(n: i32) -> i32 {
    let mut f = [0i32; 32];
    f[0] = 1;
    f[1] = 2;
    for i in 2..32 {
        f[i] = f[i - 1] + f[i - 2];
    }
    let mut ans = 0;
    let mut prev = 0;
    for k in (0..31).rev() {
        if n & (1 << k) != 0 {
            ans += f[k];
            if prev == 1 {
                return ans;
            }
            prev = 1;
        } else {
            prev = 0;
        }
    }
    ans + 1
}

fn main() {
    println!("{}", find_integers(5));
}

#[cfg(test)]
mod tests {
    use super::find_integers;

    #[test]
    fn example_one() {
        assert_eq!(find_integers(5), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_integers(1), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_integers(2), 3);
    }
}
