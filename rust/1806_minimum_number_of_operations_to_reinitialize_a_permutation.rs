/// LeetCode #1806 - Minimum Number of Operations to Reinitialize a Permutation
fn reinitialize_permutation(n: i32) -> i32 {
    let half = n >> 1;
    let mut ans = 0i32;
    let mut i = 1i32;
    loop {
        ans += 1;
        if i < half {
            i <<= 1;
        } else {
            i = ((i - half) << 1) | 1;
        }
        if i == 1 {
            return ans;
        }
    }
}

fn main() {
    println!("{}", reinitialize_permutation(2));
}

#[cfg(test)]
mod tests {
    use super::reinitialize_permutation;

    #[test]
    fn example_one() {
        assert_eq!(reinitialize_permutation(2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(reinitialize_permutation(4), 2);
    }
}
