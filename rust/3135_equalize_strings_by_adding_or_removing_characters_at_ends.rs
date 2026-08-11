/// LeetCode #3135 - Equalize Strings by Adding or Removing Characters at Ends
fn min_operations(initial: String, target: String) -> i32 {
    let a: Vec<u8> = initial.bytes().collect();
    let b: Vec<u8> = target.bytes().collect();
    let m = a.len();
    let n = b.len();
    let mut f = vec![vec![0; n + 1]; m + 1];
    let mut mx = 0;
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                f[i][j] = f[i - 1][j - 1] + 1;
                mx = mx.max(f[i][j]);
            }
        }
    }
    (m + n) as i32 - mx * 2
}

fn main() {
    println!("{}", min_operations("abcde".into(), "cdef".into()));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations("abcde".into(), "cdef".into()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations("axxy".into(), "yabx".into()), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations("xyz".into(), "xyz".into()), 0);
    }
}
