/// LeetCode #2646 - Minimize the Total Price of the Trips
fn minimum_total_price(
    n: i32,
    edges: Vec<Vec<i32>>,
    price: Vec<i32>,
    trips: Vec<Vec<i32>>,
) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }
    let mut cnt = vec![0i32; n];

    fn dfs(i: usize, fa: i32, k: usize, g: &[Vec<usize>], cnt: &mut [i32]) -> bool {
        cnt[i] += 1;
        if i == k {
            return true;
        }
        let mut ok = false;
        for &j in &g[i] {
            if j as i32 != fa {
                ok = dfs(j, i as i32, k, g, cnt);
                if ok {
                    break;
                }
            }
        }
        if !ok {
            cnt[i] -= 1;
        }
        ok
    }

    for t in &trips {
        dfs(t[0] as usize, -1, t[1] as usize, &g, &mut cnt);
    }

    fn dfs2(i: usize, fa: i32, g: &[Vec<usize>], price: &[i32], cnt: &[i32]) -> (i32, i32) {
        let mut a = cnt[i] * price[i];
        let mut b = a >> 1;
        for &j in &g[i] {
            if j as i32 != fa {
                let (x, y) = dfs2(j, i as i32, g, price, cnt);
                a += x.min(y);
                b += x;
            }
        }
        (a, b)
    }

    let (a, b) = dfs2(0, -1, &g, &price, &cnt);
    a.min(b)
}

fn main() {
    println!(
        "{}",
        minimum_total_price(
            4,
            vec![vec![0, 1], vec![1, 2], vec![1, 3]],
            vec![2, 2, 10, 6],
            vec![vec![0, 3], vec![2, 1], vec![2, 3]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_total_price;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_total_price(
                4,
                vec![vec![0, 1], vec![1, 2], vec![1, 3]],
                vec![2, 2, 10, 6],
                vec![vec![0, 3], vec![2, 1], vec![2, 3]]
            ),
            23
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_total_price(2, vec![vec![0, 1]], vec![2, 2], vec![vec![0, 0]]),
            1
        );
    }
}
