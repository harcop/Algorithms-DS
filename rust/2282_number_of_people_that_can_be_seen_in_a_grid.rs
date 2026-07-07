/// LeetCode #2282 - Number of People That Can Be Seen in a Grid
fn see_people(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = heights.len();
    let n = heights[0].len();
    let mut ans = vec![vec![0; n]; m];

    for i in 0..m {
        let mut stack: Vec<usize> = Vec::new();
        for j in 0..n {
            let mut has_equal_height = false;
            while let Some(&top) = stack.last() {
                if heights[i][top] <= heights[i][j] {
                    if heights[i][top] == heights[i][j] {
                        has_equal_height = true;
                    }
                    ans[i][stack.pop().unwrap()] += 1;
                } else {
                    break;
                }
            }
            if !stack.is_empty() && !has_equal_height {
                ans[i][*stack.last().unwrap()] += 1;
            }
            stack.push(j);
        }
    }

    for j in 0..n {
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..m {
            let mut has_equal_height = false;
            while let Some(&top) = stack.last() {
                if heights[top][j] <= heights[i][j] {
                    if heights[top][j] == heights[i][j] {
                        has_equal_height = true;
                    }
                    ans[stack.pop().unwrap()][j] += 1;
                } else {
                    break;
                }
            }
            if !stack.is_empty() && !has_equal_height {
                ans[*stack.last().unwrap()][j] += 1;
            }
            stack.push(i);
        }
    }

    ans
}

fn main() {
    println!("{:?}", see_people(vec![vec![3, 1, 4, 2, 5]]));
}

#[cfg(test)]
mod tests {
    use super::see_people;

    #[test]
    fn example_one() {
        assert_eq!(see_people(vec![vec![3, 1, 4, 2, 5]]), vec![vec![2, 1, 2, 1, 0]]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            see_people(vec![vec![5, 1], vec![3, 1], vec![4, 1]]),
            vec![vec![3, 1], vec![2, 1], vec![1, 0]]
        );
    }
}
