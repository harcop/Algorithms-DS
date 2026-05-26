/// LeetCode #1437 - Check If All 1s Are At Least Length K Places Away
fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut last = -k - 1;
    for (i, &x) in nums.iter().enumerate() {
        if x == 1 {
            if (i as i32 - last - 1) < k {
                return false;
            }
            last = i as i32;
        }
    }
    true
}

fn main() {
    println!("{}", k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::k_length_apart;

    #[test]
    fn example_one() {
        assert!(k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2));
    }

    #[test]
    fn example_two() {
        assert!(!k_length_apart(vec![1, 0, 0, 1, 0, 1], 2));
    }
}

