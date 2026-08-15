/// LeetCode #3231 - Minimum Number of Increasing Subsequence to Be Removed
fn min_operations(nums: Vec<i32>) -> i32 {
    let mut g = Vec::new();
    for &x in nums.iter() {
        let mut l = 0;
        let mut r = g.len();
        while l < r {
            let mid = (l + r) / 2;
            if g[mid] < x {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        if l == g.len() {
            g.push(x);
        } else {
            g[l] = x;
        }
    }
    g.len() as i32
}

fn main() {
    println!("{}", min_operations(vec![5, 3, 1, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![5, 3, 1, 4, 2]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![1, 2, 3, 4, 5]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![5, 4, 3, 2, 1]), 5);
    }
}
