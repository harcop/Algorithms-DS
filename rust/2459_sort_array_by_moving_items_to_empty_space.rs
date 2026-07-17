/// LeetCode #2459 - Sort Array by Moving Items to Empty Space
fn sort_array(nums: Vec<i32>) -> i32 {
    fn operations(permutation: &[usize], empty_target: usize) -> i32 {
        let mut visited = vec![false; permutation.len()];
        let mut answer = 0;

        for start in 0..permutation.len() {
            if visited[start] || permutation[start] == start {
                continue;
            }

            answer += 1;
            let mut current = start;
            while !visited[current] {
                visited[current] = true;
                answer += 1;
                current = permutation[current];
            }
        }

        if permutation[empty_target] != empty_target {
            answer -= 2;
        }
        answer
    }

    let n = nums.len();
    let empty_first: Vec<usize> = nums.iter().map(|&value| value as usize).collect();
    let empty_last: Vec<usize> = nums
        .iter()
        .map(|&value| (value as usize + n - 1) % n)
        .collect();

    operations(&empty_first, 0).min(operations(&empty_last, n - 1))
}

fn main() {
    println!("{}", sort_array(vec![4, 2, 0, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::sort_array;

    #[test]
    fn example_one() {
        assert_eq!(sort_array(vec![4, 2, 0, 3, 1]), 3);
    }

    #[test]
    fn either_empty_position_is_sorted() {
        assert_eq!(sort_array(vec![0, 1, 2, 3]), 0);
        assert_eq!(sort_array(vec![1, 2, 3, 0]), 0);
    }
}
