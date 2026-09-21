/// LeetCode #3862 - Find the Smallest Balanced Index
fn smallest_balanced_index(nums: Vec<i32>) -> i32 {
    let mut s: i128 = nums.iter().map(|&x| x as i128).sum();
    let mut p: i128 = 1;
    for i in (0..nums.len()).rev() {
        s -= nums[i] as i128;
        if s == p {
            return i as i32;
        }
        p = p.saturating_mul(nums[i] as i128);
        if p >= s {
            break;
        }
    }
    -1
}

fn main() {
    println!("{}", smallest_balanced_index(vec![2, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::smallest_balanced_index;

    #[test]
    fn example1() {
        assert_eq!(smallest_balanced_index(vec![2, 1, 2]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(smallest_balanced_index(vec![2, 8, 2, 2, 5]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(smallest_balanced_index(vec![1]), -1);
    }
}
