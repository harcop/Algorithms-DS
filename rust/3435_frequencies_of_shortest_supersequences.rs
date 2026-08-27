/// LeetCode #3435 - Frequencies of Shortest Supersequences
fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
    let mut char_to_int = [-1i32; 26];
    let mut int_to_char = vec![0usize; 26];
    let mut indegree: Vec<i32> = Vec::new();
    let mut adj = vec![Vec::new(); 26];

    let mut f = |x: u8, char_to_int: &mut [i32; 26], int_to_char: &mut [usize], indegree: &mut Vec<i32>| -> usize {
        let x = (x - b'a') as usize;
        if char_to_int[x] == -1 {
            int_to_char[indegree.len()] = x;
            char_to_int[x] = indegree.len() as i32;
            indegree.push(0);
        }
        char_to_int[x] as usize
    };

    for w in &words {
        let b = w.as_bytes();
        let u = f(b[0], &mut char_to_int, &mut int_to_char, &mut indegree);
        let v = f(b[1], &mut char_to_int, &mut int_to_char, &mut indegree);
        adj[u].push(v);
        indegree[v] += 1;
    }

    let k = indegree.len();
    let mut best = i32::MAX;
    let mut best_cnts: Vec<Vec<i32>> = Vec::new();

    for mask in 0..(1 << k) {
        let cnt: Vec<i32> = (0..k)
            .map(|i| if (mask >> i) & 1 == 1 { 2 } else { 1 })
            .collect();
        let total: i32 = cnt.iter().sum();
        if total > best {
            continue;
        }
        let mut new_cnt = cnt.clone();
        let mut new_indegree = indegree.clone();
        let mut lookup = vec![false; k];
        let mut q = Vec::new();
        for u in 0..k {
            if new_indegree[u] == 0 || new_cnt[u] == 2 {
                new_cnt[u] -= 1;
                lookup[u] = true;
                q.push(u);
            }
        }
        while !q.is_empty() {
            let mut nq = Vec::new();
            for &u in &q {
                for &v in &adj[u] {
                    new_indegree[v] -= 1;
                    if new_indegree[v] != 0 {
                        continue;
                    }
                    new_cnt[v] -= 1;
                    if lookup[v] {
                        continue;
                    }
                    lookup[v] = true;
                    nq.push(v);
                }
            }
            q = nq;
        }
        if new_cnt.iter().any(|&x| x != 0) {
            continue;
        }
        if total < best {
            best = total;
            best_cnts.clear();
        }
        best_cnts.push(cnt);
    }

    let mut result = Vec::new();
    for cnt in best_cnts {
        let mut freq = vec![0; 26];
        for (i, x) in cnt.into_iter().enumerate() {
            freq[int_to_char[i]] = x;
        }
        result.push(freq);
    }
    result
}

fn main() {
    println!("{:?}", supersequences(vec!["ab".into(), "ba".into()]));
}

#[cfg(test)]
mod tests {
    use super::supersequences;
    use std::collections::HashSet;

    fn as_set(v: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
        v.into_iter().collect()
    }

    #[test]
    fn example1() {
        assert_eq!(
            as_set(supersequences(vec!["ab".into(), "ba".into()])),
            as_set(vec![
                vec![1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ])
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            as_set(supersequences(vec!["aa".into(), "ac".into()])),
            as_set(vec![vec![
                2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]])
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            as_set(supersequences(vec!["aa".into(), "bb".into(), "cc".into()])),
            as_set(vec![vec![
                2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]])
        );
    }
}
