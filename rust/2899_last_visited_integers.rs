/// LeetCode #2899 - Last Visited Integers
fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
    let mut seen = Vec::new();
    let mut ans = Vec::new();
    let mut k = 0usize;
    for x in nums {
        if x == -1 {
            k += 1;
            ans.push(if k > seen.len() {
                -1
            } else {
                seen[seen.len() - k]
            });
        } else {
            k = 0;
            seen.push(x);
        }
    }
    ans
}

fn main() {
    println!("{:?}", last_visited_integers(vec![1, 2, -1, -1, -1]));
}

#[cfg(test)]
mod tests {
    use super::last_visited_integers;

    #[test]
    fn example_one() {
        assert_eq!(
            last_visited_integers(vec![1, 2, -1, -1, -1]),
            vec![2, 1, -1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            last_visited_integers(vec![1, -1, 2, -1, -1]),
            vec![1, 2, 1]
        );
    }
}
