/// LeetCode #839 - Similar String Groups
fn num_similar_groups(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let mut parent: Vec<usize> = (0..n).collect();

    fn find(p: &mut [usize], x: usize) -> usize {
        if p[x] != x {
            let r = find(p, p[x]);
            p[x] = r;
        }
        p[x]
    }

    fn union(p: &mut [usize], a: usize, b: usize) {
        let ra = find(p, a);
        let rb = find(p, b);
        if ra != rb {
            p[rb] = ra;
        }
    }

    fn similar(a: &[u8], b: &[u8]) -> bool {
        let mut diff = 0;
        for i in 0..a.len() {
            if a[i] != b[i] {
                diff += 1;
                if diff > 2 {
                    return false;
                }
            }
        }
        diff == 2 || diff == 0
    }

    for i in 0..n {
        for j in i + 1..n {
            if similar(strs[i].as_bytes(), strs[j].as_bytes()) {
                union(&mut parent, i, j);
            }
        }
    }
    let mut roots = std::collections::HashSet::new();
    for i in 0..n {
        roots.insert(find(&mut parent, i));
    }
    roots.len() as i32
}

fn main() {
    println!(
        "{}",
        num_similar_groups(vec!["tars".into(), "rats".into(), "arts".into(), "star".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::num_similar_groups;

    #[test]
    fn example_one() {
        assert_eq!(
            num_similar_groups(vec!["tars".into(), "rats".into(), "arts".into(), "star".into()]),
            1
        );
    }
}
