/// LeetCode #2943 - Maximize Area of Square Hole in Grid
fn maximize_square_hole_area(n: i32, m: i32, mut h_bars: Vec<i32>, mut v_bars: Vec<i32>) -> i32 {
    let _ = (n, m);
    fn f(nums: &mut [i32]) -> i32 {
        if nums.is_empty() {
            return 1;
        }
        nums.sort_unstable();
        let mut ans = 1;
        let mut cnt = 1;
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] + 1 {
                cnt += 1;
                ans = ans.max(cnt);
            } else {
                cnt = 1;
            }
        }
        ans + 1
    }
    let side = f(&mut h_bars).min(f(&mut v_bars));
    side * side
}

fn main() {
    println!("{}", maximize_square_hole_area(2, 1, vec![2, 3], vec![2]));
}

#[cfg(test)]
mod tests {
    use super::maximize_square_hole_area;

    #[test]
    fn example_one() {
        assert_eq!(maximize_square_hole_area(2, 1, vec![2, 3], vec![2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximize_square_hole_area(1, 1, vec![2], vec![2]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 3, 4]),
            9
        );
    }
}
