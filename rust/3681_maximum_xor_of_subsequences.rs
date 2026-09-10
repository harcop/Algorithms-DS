/// LeetCode #3681 - Maximum XOR of Subsequences
fn max_xor_subsequences(nums: Vec<i32>) -> i32 {
    let mut basis = [0i32; 32];
    for mut x in nums {
        for i in (0..32).rev() {
            if (x >> i) & 1 == 0 {
                continue;
            }
            if basis[i] == 0 {
                basis[i] = x;
                break;
            }
            x ^= basis[i];
        }
    }
    let mut res = 0;
    for i in (0..32).rev() {
        if (res >> i) & 1 == 0 {
            res ^= basis[i];
        }
    }
    res
}

fn main() {
    println!("{}", max_xor_subsequences(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_xor_subsequences;

    #[test]
    fn example1() {
        assert_eq!(max_xor_subsequences(vec![1, 2, 3]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_xor_subsequences(vec![5, 2]), 7);
    }
}
