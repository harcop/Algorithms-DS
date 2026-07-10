/// LeetCode #2343 - Query Kth Smallest Trimmed Number
fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let k = q[0] as usize;
        let trim = q[1] as usize;
        let mut t: Vec<(String, usize)> = nums
            .iter()
            .enumerate()
            .map(|(i, s)| (s[s.len() - trim..].to_string(), i))
            .collect();
        t.sort();
        ans.push(t[k - 1].1 as i32);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        smallest_trimmed_numbers(
            vec![
                "102".to_string(),
                "473".to_string(),
                "251".to_string(),
                "814".to_string()
            ],
            vec![vec![1, 1], vec![2, 3], vec![4, 2], vec![1, 2]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::smallest_trimmed_numbers;

    #[test]
    fn example_one() {
        assert_eq!(
            smallest_trimmed_numbers(
                vec![
                    "102".to_string(),
                    "473".to_string(),
                    "251".to_string(),
                    "814".to_string()
                ],
                vec![vec![1, 1], vec![2, 3], vec![4, 2], vec![1, 2]]
            ),
            vec![2, 2, 1, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            smallest_trimmed_numbers(
                vec![
                    "24".to_string(),
                    "37".to_string(),
                    "96".to_string(),
                    "04".to_string()
                ],
                vec![vec![2, 1], vec![2, 2]]
            ),
            vec![3, 0]
        );
    }
}
