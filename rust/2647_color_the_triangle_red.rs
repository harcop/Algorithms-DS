/// LeetCode #2647 - Color the Triangle Red
fn color_red(n: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![vec![1, 1]];
    let mut k = 0;
    let mut i = n;
    while i > 1 {
        if k == 0 {
            let mut j = 1;
            while j < (i << 1) {
                ans.push(vec![i, j]);
                j += 2;
            }
        } else if k == 1 {
            ans.push(vec![i, 2]);
        } else if k == 2 {
            let mut j = 3;
            while j < (i << 1) {
                ans.push(vec![i, j]);
                j += 2;
            }
        } else {
            ans.push(vec![i, 1]);
        }
        k = (k + 1) % 4;
        i -= 1;
    }
    ans
}

fn main() {
    println!("{:?}", color_red(2));
}

#[cfg(test)]
mod tests {
    use super::color_red;

    #[test]
    fn example_one() {
        // Multiple valid answers; this matches the pattern-based solution.
        assert_eq!(
            color_red(3),
            vec![
                vec![1, 1],
                vec![3, 1],
                vec![3, 3],
                vec![3, 5],
                vec![2, 2]
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            color_red(2),
            vec![vec![1, 1], vec![2, 1], vec![2, 3]]
        );
    }
}
