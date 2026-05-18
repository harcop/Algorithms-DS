/// LeetCode #990 - Satisfiability of Equality Equations
fn equations_possible(equations: Vec<String>) -> bool {
    let mut parent: Vec<usize> = (0..26).collect();
    fn find(p: &mut [usize], x: usize) -> usize {
        if p[x] != x { p[x] = find(p, p[x]); }
        p[x]
    }
    fn unite(p: &mut [usize], a: usize, b: usize) {
        let ra = find(p, a);
        let rb = find(p, b);
        p[ra] = rb;
    }
    for eq in &equations {
        let a = (eq.as_bytes()[0] - b'a') as usize;
        let b = (eq.as_bytes()[3] - b'a') as usize;
        if eq.as_bytes()[1] == b'=' {
            unite(&mut parent, a, b);
        }
    }
    for eq in equations {
        let a = (eq.as_bytes()[0] - b'a') as usize;
        let b = (eq.as_bytes()[3] - b'a') as usize;
        if eq.as_bytes()[1] == b'!' && find(&mut parent, a) == find(&mut parent, b) {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        equations_possible(vec!["a==b".into(), "b!=a".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::equations_possible;

    #[test]
    fn example_one() {
        assert!(!equations_possible(vec!["a==b".into(), "b!=a".into()]));
    }

    #[test]
    fn example_two() {
        assert!(equations_possible(vec!["b==a".into(), "a==b".into()]));
    }
}
