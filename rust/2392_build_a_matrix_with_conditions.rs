/// LeetCode #2392 - Build a Matrix With Conditions
use std::collections::VecDeque;

fn build_matrix(
    k: i32,
    row_conditions: Vec<Vec<i32>>,
    col_conditions: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let k = k as usize;

    fn topo(k: usize, cond: &[Vec<i32>]) -> Option<Vec<usize>> {
        let mut g = vec![vec![]; k + 1];
        let mut indeg = vec![0; k + 1];
        for e in cond {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            indeg[b] += 1;
        }
        let mut q: VecDeque<usize> = (1..=k).filter(|&i| indeg[i] == 0).collect();
        let mut res = Vec::with_capacity(k);
        while let Some(i) = q.pop_front() {
            res.push(i);
            for &j in &g[i] {
                indeg[j] -= 1;
                if indeg[j] == 0 {
                    q.push_back(j);
                }
            }
        }
        if res.len() == k {
            Some(res)
        } else {
            None
        }
    }

    let row = match topo(k, &row_conditions) {
        Some(r) => r,
        None => return vec![],
    };
    let col = match topo(k, &col_conditions) {
        Some(c) => c,
        None => return vec![],
    };

    let mut col_pos = vec![0; k + 1];
    for (i, &v) in col.iter().enumerate() {
        col_pos[v] = i;
    }
    let mut ans = vec![vec![0; k]; k];
    for (i, &v) in row.iter().enumerate() {
        ans[i][col_pos[v]] = v as i32;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        build_matrix(3, vec![vec![1, 2], vec![3, 2]], vec![vec![2, 1], vec![3, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::build_matrix;
    use std::collections::HashMap;

    fn positions(mat: &[Vec<i32>]) -> HashMap<i32, (usize, usize)> {
        let mut pos = HashMap::new();
        for (i, row) in mat.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v != 0 {
                    pos.insert(v, (i, j));
                }
            }
        }
        pos
    }

    fn satisfies(mat: &[Vec<i32>], k: i32, rows: &[Vec<i32>], cols: &[Vec<i32>]) -> bool {
        if mat.is_empty() {
            return false;
        }
        let pos = positions(mat);
        if pos.len() != k as usize {
            return false;
        }
        for e in rows {
            let (ra, _) = pos[&e[0]];
            let (rb, _) = pos[&e[1]];
            if ra >= rb {
                return false;
            }
        }
        for e in cols {
            let (_, ca) = pos[&e[0]];
            let (_, cb) = pos[&e[1]];
            if ca >= cb {
                return false;
            }
        }
        true
    }

    #[test]
    fn example_one() {
        let rows = vec![vec![1, 2], vec![3, 2]];
        let cols = vec![vec![2, 1], vec![3, 2]];
        let ans = build_matrix(3, rows.clone(), cols.clone());
        assert!(satisfies(&ans, 3, &rows, &cols));
    }

    #[test]
    fn example_two() {
        assert!(build_matrix(
            3,
            vec![vec![1, 2], vec![2, 3], vec![3, 1], vec![2, 3]],
            vec![vec![2, 1]]
        )
        .is_empty());
    }
}
