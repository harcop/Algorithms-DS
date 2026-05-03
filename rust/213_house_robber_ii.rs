/// LeetCode #213 - House Robber II
fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    fn linear(slice: &[i32]) -> i32 {
        let (mut a, mut b) = (0, 0);
        for &x in slice {
            let nb = b.max(a + x);
            a = b;
            b = nb;
        }
        a.max(b)
    }
    linear(&nums[..n - 1]).max(linear(&nums[1..]))
}

fn main() {
    println!("{}", rob(vec![2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::rob;

    #[test]
    fn example_one() {
        assert_eq!(rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(rob(vec![1, 2, 3]), 3);
    }
}
