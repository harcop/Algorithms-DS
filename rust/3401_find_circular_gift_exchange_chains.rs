/// LeetCode #3401 - Find Circular Gift Exchange Chains (SQL; Rust analogue)
/// secret_santa: (giver_id, receiver_id, gift_value)
fn find_circular_chains(secret_santa: Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    use std::collections::{HashMap, HashSet};
    let mut nxt: HashMap<i32, (i32, i32)> = HashMap::new();
    for (g, r, v) in secret_santa {
        nxt.insert(g, (r, v));
    }
    let mut seen = HashSet::new();
    let mut chains = Vec::new();
    let starts: Vec<i32> = nxt.keys().copied().collect();
    for start in starts {
        if !seen.insert(start) {
            continue;
        }
        let mut cur = start;
        let mut len = 0i32;
        let mut sum = 0i32;
        loop {
            let Some(&(to, v)) = nxt.get(&cur) else {
                break;
            };
            len += 1;
            sum += v;
            cur = to;
            if cur == start {
                break;
            }
            seen.insert(cur);
        }
        if len > 0 {
            chains.push((len, sum));
        }
    }
    chains.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
    chains
        .into_iter()
        .enumerate()
        .map(|(i, (len, sum))| ((i + 1) as i32, len, sum))
        .collect()
}

fn main() {
    println!(
        "{:?}",
        find_circular_chains(vec![
            (1, 2, 20),
            (2, 3, 30),
            (3, 1, 40),
            (4, 5, 25),
            (5, 4, 35)
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::find_circular_chains;

    #[test]
    fn example() {
        assert_eq!(
            find_circular_chains(vec![
                (1, 2, 20),
                (2, 3, 30),
                (3, 1, 40),
                (4, 5, 25),
                (5, 4, 35)
            ]),
            vec![(1, 3, 90), (2, 2, 60)]
        );
    }
}
