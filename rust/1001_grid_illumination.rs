/// LeetCode #1001 - Grid Illumination
use std::collections::HashMap;

fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut row: HashMap<i32, i32> = HashMap::new();
    let mut col: HashMap<i32, i32> = HashMap::new();
    let mut diag1: HashMap<i32, i32> = HashMap::new();
    let mut diag2: HashMap<i32, i32> = HashMap::new();
    let mut lamps_set: HashMap<(i32, i32), i32> = HashMap::new();
    for l in lamps {
        let r = l[0];
        let c = l[1];
        *lamps_set.entry((r, c)).or_default() += 1;
        *row.entry(r).or_default() += 1;
        *col.entry(c).or_default() += 1;
        *diag1.entry(r - c).or_default() += 1;
        *diag2.entry(r + c).or_default() += 1;
    }
    let mut out = Vec::new();
    for q in queries {
        let r = q[0];
        let c = q[1];
        let lit = (r - 1..=r + 1).any(|rr| {
            (c - 1..=c + 1).any(|cc| lamps_set.contains_key(&(rr, cc)))
        });
        out.push(if lit { 1 } else { 0 });
        for rr in r - 1..=r + 1 {
            for cc in c - 1..=c + 1 {
                if let Some(cnt) = lamps_set.get_mut(&(rr, cc)) {
                    if *cnt > 0 {
                        *row.entry(rr).or_default() -= *cnt;
                        *col.entry(cc).or_default() -= *cnt;
                        *diag1.entry(rr - cc).or_default() -= *cnt;
                        *diag2.entry(rr + cc).or_default() -= *cnt;
                        *cnt = 0;
                    }
                }
            }
        }
    }
    let _ = n;
    out
}

fn main() {
    println!(
        "{:?}",
        grid_illumination(5, vec![vec![0, 0], vec![4, 4]], vec![vec![1, 1], vec![1, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::grid_illumination;

    #[test]
    fn example_one() {
        assert_eq!(
            grid_illumination(5, vec![vec![0, 0], vec![4, 4]], vec![vec![1, 1], vec![1, 0]]),
            vec![1, 1]
        );
    }
}
