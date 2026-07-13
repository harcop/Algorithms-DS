/// LeetCode #2382 - Maximum Segment Sum After Removals
fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
    let n = nums.len();
    let mut parent: Vec<usize> = (0..n).collect();
    let mut sum = vec![0i64; n];
    let mut ans = vec![0i64; n];
    let mut mx = 0i64;

    fn find(x: usize, parent: &mut [usize]) -> usize {
        if parent[x] != x {
            parent[x] = find(parent[x], parent);
        }
        parent[x]
    }

    fn merge(a: usize, b: usize, parent: &mut [usize], sum: &mut [i64]) {
        let pa = find(a, parent);
        let pb = find(b, parent);
        parent[pa] = pb;
        sum[pb] += sum[pa];
    }

    for j in (1..n).rev() {
        let i = remove_queries[j] as usize;
        sum[i] = nums[i] as i64;
        if i > 0 && sum[find(i - 1, &mut parent)] > 0 {
            merge(i, i - 1, &mut parent, &mut sum);
        }
        if i + 1 < n && sum[find(i + 1, &mut parent)] > 0 {
            merge(i, i + 1, &mut parent, &mut sum);
        }
        mx = mx.max(sum[find(i, &mut parent)]);
        ans[j - 1] = mx;
    }

    ans
}

fn main() {
    println!(
        "{:?}",
        maximum_segment_sum(vec![1, 2, 5, 6, 1], vec![0, 3, 2, 4, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_segment_sum;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_segment_sum(vec![1, 2, 5, 6, 1], vec![0, 3, 2, 4, 1]),
            vec![14, 7, 2, 2, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_segment_sum(vec![3, 2, 11, 1], vec![3, 2, 1, 0]),
            vec![16, 5, 3, 0]
        );
    }
}
