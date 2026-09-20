/// LeetCode #3840 - House Robber V
fn rob(nums: Vec<i32>, colors: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut f: i64 = 0;
    let mut g: i64 = nums[0] as i64;
    for i in 1..n {
        if colors[i - 1] == colors[i] {
            let gg = f + nums[i] as i64;
            f = f.max(g);
            g = gg;
        } else {
            let gg = f.max(g) + nums[i] as i64;
            f = f.max(g);
            g = gg;
        }
    }
    f.max(g)
}

fn main() {
    println!("{}", rob(vec![1, 4, 3, 5], vec![1, 1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::rob;

    #[test]
    fn example1() {
        assert_eq!(rob(vec![1, 4, 3, 5], vec![1, 1, 2, 2]), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(rob(vec![3, 1, 2, 4], vec![2, 3, 2, 2]), 8);
    }

    #[test]
    fn example3() {
        assert_eq!(rob(vec![10, 1, 3, 9], vec![1, 1, 1, 2]), 22);
    }
}
