/// LeetCode #2307 - Check for Contradictions in Equations
use std::collections::HashMap;

fn check_contradictions(equations: Vec<Vec<String>>, values: Vec<f64>) -> bool {
    let mut d: HashMap<String, usize> = HashMap::new();
    let mut n = 0usize;
    for e in &equations {
        for s in e {
            if !d.contains_key(s) {
                d.insert(s.clone(), n);
                n += 1;
            }
        }
    }

    let mut p: Vec<usize> = (0..n).collect();
    let mut w = vec![1.0f64; n];

    fn find(x: usize, p: &mut [usize], w: &mut [f64]) -> usize {
        if p[x] != x {
            let root = find(p[x], p, w);
            w[x] *= w[p[x]];
            p[x] = root;
        }
        p[x]
    }

    for (i, e) in equations.iter().enumerate() {
        let a = d[&e[0]];
        let b = d[&e[1]];
        let v = values[i];
        let pa = find(a, &mut p, &mut w);
        let pb = find(b, &mut p, &mut w);
        if pa != pb {
            p[pb] = pa;
            w[pb] = v * w[a] / w[b];
        } else if (v * w[a] - w[b]).abs() >= 1e-5 {
            return true;
        }
    }
    false
}

fn main() {
    println!(
        "{}",
        check_contradictions(
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["b".to_string(), "c".to_string()],
                vec!["a".to_string(), "c".to_string()]
            ],
            vec![3.0, 0.5, 1.5]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::check_contradictions;

    #[test]
    fn example_one() {
        assert!(!check_contradictions(
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["b".to_string(), "c".to_string()],
                vec!["a".to_string(), "c".to_string()]
            ],
            vec![3.0, 0.5, 1.5]
        ));
    }

    #[test]
    fn example_two() {
        assert!(check_contradictions(
            vec![
                vec!["le".to_string(), "et".to_string()],
                vec!["le".to_string(), "code".to_string()],
                vec!["code".to_string(), "et".to_string()]
            ],
            vec![2.0, 5.0, 0.5]
        ));
    }
}
