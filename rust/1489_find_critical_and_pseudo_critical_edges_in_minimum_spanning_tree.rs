/// LeetCode #1489 - Find Critical And Pseudo Critical Edges In Minimum Spanning Tree
fn find_critical_and_pseudo_critical(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut es: Vec<(i32, i32, i32, i32)> = edges.iter().enumerate().map(|(i, e)| (e[2], i as i32, e[0], e[1])).collect();
    es.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    fn mst(n: usize, es: &[(i32, i32, i32, i32)], skip: Option<usize>, force: Option<usize>) -> i32 {
        let mut p: Vec<usize> = (0..n).collect();
        fn find(p: &mut [usize], x: usize) -> usize {
            if p[x] != x { p[x] = find(p, p[x]); }
            p[x]
        }
        let mut w = 0i32;
        let mut cnt = 0usize;
        if let Some(fi) = force {
            let (_, _, a, b) = es[fi];
            let (ra, rb) = (find(&mut p, a as usize), find(&mut p, b as usize));
            if ra != rb { p[ra] = rb; w += es[fi].0; cnt += 1; }
        }
        for (i, &(wt, _, a, b)) in es.iter().enumerate() {
            if Some(i) == skip || Some(i) == force { continue; }
            let (ra, rb) = (find(&mut p, a as usize), find(&mut p, b as usize));
            if ra != rb { p[ra] = rb; w += wt; cnt += 1; }
        }
        if cnt < n - 1 { i32::MAX } else { w }
    }
    let base = mst(n, &es, None, None);
    let mut crit = Vec::new();
    let mut pseudo = Vec::new();
    for i in 0..es.len() {
        let idx = es[i].1;
        if mst(n, &es, Some(i), None) > base { crit.push(idx); }
        else if mst(n, &es, None, Some(i)) == base { pseudo.push(idx); }
    }
    vec![crit, pseudo]
}
fn main() { println!("{:?}", find_critical_and_pseudo_critical(4, vec![vec![0,1,1],vec![1,2,1],vec![2,3,1],vec![0,3,1]])); }
#[cfg(test)]
mod tests {
    use super::find_critical_and_pseudo_critical;
    #[test]
    fn example_one() {
        let r = find_critical_and_pseudo_critical(4, vec![vec![0,1,1],vec![1,2,1],vec![2,3,1],vec![0,3,1]]);
        assert!(!r[0].is_empty() || !r[1].is_empty());
    }
}