/// LeetCode #2977 - Minimum Cost to Convert String II
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct Node {
    children: [Option<Rc<RefCell<Node>>>; 26],
    v: i32,
}

fn minimum_cost(
    source: String,
    target: String,
    original: Vec<String>,
    changed: Vec<String>,
    cost: Vec<i32>,
) -> i64 {
    const INF: i64 = 1_000_000_000_000_000;
    let m = cost.len();
    let size = m * 2;
    let mut g = vec![vec![INF; size]; size];
    for i in 0..size {
        g[i][i] = 0;
    }
    let root = Rc::new(RefCell::new(Node {
        children: Default::default(),
        v: -1,
    }));
    let mut idx = 0usize;

    let mut insert = |w: &str| -> usize {
        let mut node = Rc::clone(&root);
        for c in w.bytes() {
            let i = (c - b'a') as usize;
            let next = {
                let mut n = node.borrow_mut();
                if n.children[i].is_none() {
                    n.children[i] = Some(Rc::new(RefCell::new(Node {
                        children: Default::default(),
                        v: -1,
                    })));
                }
                Rc::clone(n.children[i].as_ref().unwrap())
            };
            node = next;
        }
        let mut n = node.borrow_mut();
        if n.v < 0 {
            n.v = idx as i32;
            idx += 1;
        }
        n.v as usize
    };

    for i in 0..m {
        let x = insert(&original[i]);
        let y = insert(&changed[i]);
        g[x][y] = g[x][y].min(cost[i] as i64);
    }

    for k in 0..idx {
        for i in 0..idx {
            if g[i][k] >= INF {
                continue;
            }
            for j in 0..idx {
                if g[i][k] + g[k][j] < g[i][j] {
                    g[i][j] = g[i][k] + g[k][j];
                }
            }
        }
    }

    let sb = source.as_bytes();
    let tb = target.as_bytes();
    let n = sb.len();
    let mut memo = vec![-2i64; n + 1];

    fn dfs(
        i: usize,
        sb: &[u8],
        tb: &[u8],
        root: &Rc<RefCell<Node>>,
        g: &[Vec<i64>],
        memo: &mut [i64],
    ) -> i64 {
        const INF: i64 = 1_000_000_000_000_000;
        if i >= sb.len() {
            return 0;
        }
        if memo[i] != -2 {
            return memo[i];
        }
        let mut res = if sb[i] == tb[i] {
            dfs(i + 1, sb, tb, root, g, memo)
        } else {
            INF
        };
        let mut p = Some(Rc::clone(root));
        let mut q = Some(Rc::clone(root));
        for j in i..sb.len() {
            let pi = (sb[j] - b'a') as usize;
            let qi = (tb[j] - b'a') as usize;
            p = p.and_then(|node| node.borrow().children[pi].clone());
            q = q.and_then(|node| node.borrow().children[qi].clone());
            match (&p, &q) {
                (Some(pn), Some(qn)) => {
                    let pv = pn.borrow().v;
                    let qv = qn.borrow().v;
                    if pv >= 0 && qv >= 0 {
                        let next = dfs(j + 1, sb, tb, root, g, memo);
                        res = res.min(next + g[pv as usize][qv as usize]);
                    }
                }
                _ => break,
            }
        }
        memo[i] = res;
        res
    }

    let ans = dfs(0, sb, tb, &root, &g, &mut memo);
    if ans >= INF {
        -1
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        minimum_cost(
            "abcd".into(),
            "acbe".into(),
            vec![
                "a".into(),
                "b".into(),
                "c".into(),
                "c".into(),
                "e".into(),
                "d".into()
            ],
            vec![
                "b".into(),
                "c".into(),
                "b".into(),
                "e".into(),
                "b".into(),
                "e".into()
            ],
            vec![2, 5, 5, 1, 2, 20]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_cost(
                "abcd".into(),
                "acbe".into(),
                vec![
                    "a".into(),
                    "b".into(),
                    "c".into(),
                    "c".into(),
                    "e".into(),
                    "d".into()
                ],
                vec![
                    "b".into(),
                    "c".into(),
                    "b".into(),
                    "e".into(),
                    "b".into(),
                    "e".into()
                ],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_cost(
                "abcdefgh".into(),
                "acdeeghh".into(),
                vec!["bcd".into(), "fgh".into(), "thh".into()],
                vec!["cde".into(), "thh".into(), "ghh".into()],
                vec![1, 3, 5]
            ),
            9
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            minimum_cost(
                "abcdefgh".into(),
                "addddddd".into(),
                vec!["bcd".into(), "defgh".into()],
                vec!["ddd".into(), "ddddd".into()],
                vec![100, 1578]
            ),
            -1
        );
    }
}
