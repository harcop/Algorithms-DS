/// LeetCode #1944 - Number of Visible People in a Queue
fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let n = heights.len();
    let mut ans = vec![0i32; n];
    let mut stk: Vec<i32> = Vec::new();
    for i in (0..n).rev() {
        while let Some(&top) = stk.last() {
            if top < heights[i] {
                ans[i] += 1;
                stk.pop();
            } else {
                break;
            }
        }
        if !stk.is_empty() {
            ans[i] += 1;
        }
        stk.push(heights[i]);
    }
    ans
}

fn main() {
    println!("{:?}", can_see_persons_count(vec![10, 6, 8, 5, 11, 9]));
}

#[cfg(test)]
mod tests {
    use super::can_see_persons_count;

    #[test]
    fn example_one() {
        assert_eq!(
            can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
            vec![3, 1, 2, 1, 1, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(can_see_persons_count(vec![5, 1, 2, 3, 10]), vec![4, 1, 1, 1, 0]);
    }
}
