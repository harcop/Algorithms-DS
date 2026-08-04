/// LeetCode #2971 - Find Polygon With the Largest Perimeter
fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    nums.sort_unstable();
    let mut s = vec![0i64; nums.len() + 1];
    for i in 0..nums.len() {
        s[i + 1] = s[i] + nums[i] as i64;
    }
    let mut ans = -1i64;
    for k in 3..=nums.len() {
        if s[k - 1] > nums[k - 1] as i64 {
            ans = ans.max(s[k]);
        }
    }
    ans
}

fn main() {
    println!("{}", largest_perimeter(vec![5, 5, 5]));
}

#[cfg(test)]
mod tests {
    use super::largest_perimeter;

    #[test]
    fn example_one() {
        assert_eq!(largest_perimeter(vec![5, 5, 5]), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
    }

    #[test]
    fn example_three() {
        assert_eq!(largest_perimeter(vec![5, 5, 50]), -1);
    }
}
