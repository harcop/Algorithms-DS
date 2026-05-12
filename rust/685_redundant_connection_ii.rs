/// LeetCode #685 - Redundant Connection II
fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut parent_edge: Vec<i32> = vec![-1; n + 1];
    let mut cand1: Option<usize> = None;
    let mut cand2: Option<usize> = None;

    for i in 0..n {
        let v = edges[i][1] as usize;
        if parent_edge[v] != -1 {
            cand1 = Some(parent_edge[v] as usize);
            cand2 = Some(i);
        } else {
            parent_edge[v] = i as i32;
        }
    }

    let mut p: Vec<usize> = (0..=n).collect();
    fn find(p: &mut Vec<usize>, x: usize) -> usize {
        if p[x] != x {
            let r = find(p, p[x]);
            p[x] = r;
        }
        p[x]
    }

    for i in 0..n {
        if Some(i) == cand2 {
            continue;
        }
        let u = edges[i][0] as usize;
        let v = edges[i][1] as usize;
        let ru = find(&mut p, u);
        let rv = find(&mut p, v);
        if ru == rv {
            if let Some(c1) = cand1 {
                return edges[c1].clone();
            }
            return edges[i].clone();
        }
        p[rv] = ru;
    }

    edges[cand2.unwrap()].clone()
}

fn main() {
    println!(
        "{:?}",
        find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_redundant_directed_connection;

    #[test]
    fn example_one() {
        assert_eq!(
            find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_redundant_directed_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 1],
                vec![1, 5]
            ]),
            vec![4, 1]
        );
    }
}
