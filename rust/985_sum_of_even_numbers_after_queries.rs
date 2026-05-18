/// LeetCode #985 - Sum of Even Numbers After Queries
fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut arr = nums;
    let mut sum_even: i64 = arr.iter().filter(|&&x| x % 2 == 0).map(|&x| x as i64).sum();
    let mut out = Vec::new();
    for q in queries {
        let idx = q[0] as usize;
        let val = q[1];
        if arr[idx] % 2 == 0 {
            sum_even -= arr[idx] as i64;
        }
        arr[idx] += val;
        if arr[idx] % 2 == 0 {
            sum_even += arr[idx] as i64;
        }
        out.push(sum_even as i32);
    }
    out
}

fn main() {
    println!(
        "{:?}",
        sum_even_after_queries(vec![1], vec![vec![0, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::sum_even_after_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            sum_even_after_queries(vec![1], vec![vec![0, 1]]),
            vec![0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sum_even_after_queries(vec![4], vec![vec![1, 0], vec![-3, 5]]),
            vec![4, 6]
        );
    }
}
