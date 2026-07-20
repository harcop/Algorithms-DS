/// LeetCode #2538 - Difference Between Maximum and Minimum Price Sum
fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
    let n = n as usize;
    let mut tree = vec![Vec::new(); n];
    for e in &edges {
        let (u, v) = (e[0] as usize, e[1] as usize);
        tree[u].push(v);
        tree[v].push(u);
    }
    let mut max_sums = vec![0i64; n];
    max_sum(&tree, 0, usize::MAX, &mut max_sums, &price);
    let mut ans = 0i64;
    reroot(&tree, 0, usize::MAX, 0, &max_sums, &price, &mut ans);
    ans
}

fn max_sum(
    tree: &[Vec<usize>],
    u: usize,
    prev: usize,
    max_sums: &mut [i64],
    price: &[i32],
) -> i64 {
    let mut max_child = 0i64;
    for &v in &tree[u] {
        if v != prev {
            max_child = max_child.max(max_sum(tree, v, u, max_sums, price));
        }
    }
    max_sums[u] = price[u] as i64 + max_child;
    max_sums[u]
}

fn reroot(
    tree: &[Vec<usize>],
    u: usize,
    prev: usize,
    parent_sum: i64,
    max_sums: &[i64],
    price: &[i32],
    ans: &mut i64,
) {
    let mut max1 = 0i64;
    let mut max2 = 0i64;
    let mut max_node = usize::MAX;
    for &v in &tree[u] {
        if v == prev {
            continue;
        }
        if max_sums[v] > max1 {
            max2 = max1;
            max1 = max_sums[v];
            max_node = v;
        } else if max_sums[v] > max2 {
            max2 = max_sums[v];
        }
    }
    if tree[u].len() == 1 {
        *ans = (*ans).max(parent_sum).max(max1);
    }
    for &v in &tree[u] {
        if v == prev {
            continue;
        }
        let next = if v == max_node {
            price[u] as i64 + parent_sum.max(max2)
        } else {
            price[u] as i64 + parent_sum.max(max1)
        };
        reroot(tree, v, u, next, max_sums, price, ans);
    }
}

fn main() {
    println!(
        "{}",
        max_output(
            6,
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]],
            vec![9, 8, 7, 6, 10, 5]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_output;

    #[test]
    fn example_one() {
        assert_eq!(
            max_output(
                6,
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]],
                vec![9, 8, 7, 6, 10, 5]
            ),
            24
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_output(3, vec![vec![0, 1], vec![1, 2]], vec![1, 1, 1]),
            2
        );
    }
}
