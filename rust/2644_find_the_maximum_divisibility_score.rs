/// LeetCode #2644 - Find the Maximum Divisibility Score
fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
    let mut ans = divisors[0];
    let mut mx = 0;
    for &div in &divisors {
        let cnt = nums.iter().filter(|&&x| x % div == 0).count() as i32;
        if mx < cnt {
            mx = cnt;
            ans = div;
        } else if mx == cnt && ans > div {
            ans = div;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_div_score(vec![2, 9, 15, 50], vec![5, 3, 7, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::max_div_score;

    #[test]
    fn example_one() {
        assert_eq!(max_div_score(vec![2, 9, 15, 50], vec![5, 3, 7, 2]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3]), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_div_score(vec![20, 14, 21, 10], vec![10, 16, 20]), 10);
    }
}
