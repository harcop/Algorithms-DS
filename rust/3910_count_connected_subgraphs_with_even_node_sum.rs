/// LeetCode #3910 - Count Connected Subgraphs with Even Node Sum
fn even_sum_subgraphs(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut adj = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut ans = 0i32;
    for mask in 1u32..(1 << n) {
        let mut sum = 0i32;
        let mut start = 0usize;
        let mut cnt = 0u32;
        for i in 0..n {
            if mask & (1 << i) != 0 {
                sum += nums[i];
                start = i;
                cnt += 1;
            }
        }
        if sum % 2 != 0 {
            continue;
        }
        let mut seen = 1u32 << start;
        let mut stack = vec![start];
        while let Some(u) = stack.pop() {
            for &v in &adj[u] {
                if mask & (1 << v) != 0 && seen & (1 << v) == 0 {
                    seen |= 1 << v;
                    stack.push(v);
                }
            }
        }
        if seen.count_ones() == cnt {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", even_sum_subgraphs(vec![1, 0, 1], vec![vec![0, 1], vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::even_sum_subgraphs;

    #[test]
    fn example1() {
        assert_eq!(
            even_sum_subgraphs(vec![1, 0, 1], vec![vec![0, 1], vec![1, 2]]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(even_sum_subgraphs(vec![1], vec![]), 0);
    }
}
