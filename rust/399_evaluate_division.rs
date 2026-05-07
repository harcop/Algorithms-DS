/// LeetCode #399 - Evaluate Division (weighted DSU snapshot per query isn't needed — path compress with mut refs)
use std::collections::HashMap;

fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut mp: HashMap<String, usize> = HashMap::new();
    let mut sz = 0usize;
    for e in &equations {
        for side in &[e[0].clone(), e[1].clone()] {
            mp.entry(side.clone()).or_insert_with(|| {
                sz += 1;
                sz - 1
            });
        }
    }
    let n = sz.max(1);
    let mut parent: Vec<usize> = (0..n).collect();
    let mut weight: Vec<f64> = vec![1.0; n];

    fn find(p: &mut [usize], w: &mut [f64], x: usize) -> usize {
        if p[x] != x {
            let r = find(p, w, p[x]);
            w[x] *= w[p[x]];
            p[x] = r;
        }
        p[x]
    }

    fn join(p: &mut [usize], w: &mut [f64], a: usize, b: usize, val: f64) {
        let ra = find(p, w, a);
        let rb = find(p, w, b);
        if ra == rb {
            return;
        }
        p[rb] = ra;
        w[rb] = val * w[a] / w[b];
    }

    for (eq, &v) in equations.iter().zip(values.iter()) {
        let &a = mp.get(&eq[0]).unwrap();
        let &b = mp.get(&eq[1]).unwrap();
        join(&mut parent, &mut weight, a, b, v);
    }

    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let ia = mp.get(&q[0]).copied();
        let ib = mp.get(&q[1]).copied();
        match (ia, ib) {
            (Some(mut a), Some(mut b)) => {
                let ra = find(&mut parent, &mut weight, a);
                let rb = find(&mut parent, &mut weight, b);
                if ra != rb {
                    ans.push(-1.0);
                } else {
                    ans.push(weight[b] / weight[a]);
                }
            }
            _ => ans.push(-1.0),
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        calc_equation(
            vec![
                vec!["a".into(), "b".into()],
                vec!["b".into(), "c".into()],
            ],
            vec![2.0, 3.0],
            vec![vec!["a".into(), "c".into()], vec!["b".into(), "a".into()]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert!(
            ((calc_equation(
                vec![
                    vec!["a".into(), "b".into()],
                    vec!["b".into(), "c".into()],
                ],
                vec![2.0, 3.0],
                vec![vec!["a".into(), "c".into()], vec!["b".into(), "a".into()]],
            ))[0]
                - 6.0)
                .abs()
                < 1e-9
        );
    }
}
