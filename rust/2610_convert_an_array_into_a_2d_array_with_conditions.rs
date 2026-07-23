/// LeetCode #2610 - Convert an Array Into a 2D Array With Conditions
fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut cnt = vec![0; n + 1];
    for &x in &nums {
        cnt[x as usize] += 1;
    }
    let mut ans: Vec<Vec<i32>> = Vec::new();
    for x in 1..=n as i32 {
        for j in 0..cnt[x as usize] {
            if ans.len() <= j {
                ans.push(Vec::new());
            }
            ans[j].push(x);
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_matrix(vec![1, 3, 4, 1, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_matrix;

    #[test]
    fn example_one() {
        let result = find_matrix(vec![1, 3, 4, 1, 2, 3, 1]);
        assert_eq!(result.len(), 3);
        assert_eq!(result.iter().map(|r| r.len()).sum::<usize>(), 7);
        for row in &result {
            let mut seen = std::collections::HashSet::new();
            for &x in row {
                assert!(seen.insert(x));
            }
        }
    }

    #[test]
    fn example_two() {
        let result = find_matrix(vec![1, 2, 3, 4]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 4);
    }
}
