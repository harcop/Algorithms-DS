/// LeetCode #1203 - Sort Items by Groups Respecting Dependencies
fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let m = m as usize;
    let mut g_group = vec![Vec::new(); m + 1];
    let mut g_item = vec![Vec::new(); n];
    for i in 0..n {
        let gi = if group[i] == -1 { m } else { group[i] as usize };
        for &dep in &before_items[i] {
            let dep = dep as usize;
            let gd = if group[dep] == -1 {
                m
            } else {
                group[dep] as usize
            };
            if gi != gd {
                g_group[gd].push(gi);
            } else {
                g_item[dep].push(i);
            }
        }
    }
    fn topo(nodes: usize, adj: &[Vec<usize>]) -> Option<Vec<usize>> {
        let mut indeg = vec![0; nodes];
        for u in 0..nodes {
            for &v in &adj[u] {
                indeg[v] += 1;
            }
        }
        let mut q = std::collections::VecDeque::new();
        for i in 0..nodes {
            if indeg[i] == 0 {
                q.push_back(i);
            }
        }
        let mut order = Vec::new();
        while let Some(u) = q.pop_front() {
            order.push(u);
            for &v in &adj[u] {
                indeg[v] -= 1;
                if indeg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if order.len() == nodes {
            Some(order)
        } else {
            None
        }
    }
    let Some(group_order) = topo(m + 1, &g_group) else {
        return vec![];
    };
    let mut ans = Vec::new();
    for &gi in &group_order {
        let items: Vec<usize> = (0..n)
            .filter(|&i| {
                if group[i] == -1 {
                    gi == m
                } else {
                    group[i] as usize == gi
                }
            })
            .collect();
        let mut sub_adj = vec![Vec::new(); items.len()];
        let idx: std::collections::HashMap<usize, usize> =
            items.iter().enumerate().map(|(j, &i)| (i, j)).collect();
        for (j, &i) in items.iter().enumerate() {
            for &dep in &before_items[i] {
                let dep = dep as usize;
                if let Some(&dj) = idx.get(&dep) {
                    sub_adj[dj].push(j);
                }
            }
        }
        let Some(item_order) = topo(items.len(), &sub_adj) else {
            return vec![];
        };
        for j in item_order {
            ans.push(items[j] as i32);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        sort_items(
            8,
            2,
            vec![-1, -1, 1, 0, 0, 1, 0, -1],
            vec![
                vec![],
                vec![6],
                vec![5],
                vec![6],
                vec![3, 6],
                vec![],
                vec![],
                vec![],
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::sort_items;

    #[test]
    fn example_one() {
        let out = sort_items(
            8,
            2,
            vec![-1, -1, 1, 0, 0, 1, 0, -1],
            vec![
                vec![],
                vec![6],
                vec![5],
                vec![6],
                vec![3, 6],
                vec![],
                vec![],
                vec![],
            ],
        );
        assert_eq!(out.len(), 8);
        assert_eq!(out, vec![6, 3, 4, 5, 2, 0, 1, 7]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sort_items(
                8,
                2,
                vec![-1, -1, 1, 0, 0, 1, 0, -1],
                vec![
                    vec![],
                    vec![6],
                    vec![5],
                    vec![6],
                    vec![3],
                    vec![],
                    vec![4],
                    vec![],
                ],
            ),
            vec![]
        );
    }
}
