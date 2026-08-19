/// LeetCode #3291 - Minimum Number of Valid Strings to Form Target I
struct Trie {
    nodes: Vec<[usize; 26]>,
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![[0; 26]],
        }
    }

    fn insert(&mut self, w: &str) {
        let mut u = 0;
        for b in w.bytes() {
            let c = (b - b'a') as usize;
            let nxt = self.nodes[u][c];
            if nxt == 0 {
                let id = self.nodes.len();
                self.nodes.push([0; 26]);
                self.nodes[u][c] = id;
                u = id;
            } else {
                u = nxt;
            }
        }
    }
}

fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    let mut trie = Trie::new();
    for w in &words {
        trie.insert(w);
    }
    let t = target.as_bytes();
    let n = t.len();
    let inf = i32::MAX / 4;
    let mut dp = vec![inf; n + 1];
    dp[n] = 0;
    for i in (0..n).rev() {
        let mut u = 0;
        for j in i..n {
            let c = (t[j] - b'a') as usize;
            if trie.nodes[u][c] == 0 {
                break;
            }
            u = trie.nodes[u][c];
            dp[i] = dp[i].min(1 + dp[j + 1]);
        }
    }
    if dp[0] >= inf {
        -1
    } else {
        dp[0]
    }
}

fn main() {
    println!(
        "{}",
        min_valid_strings(
            vec!["abc".into(), "aaaaa".into(), "bcdef".into()],
            "aabcdabc".into()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_valid_strings;

    #[test]
    fn example1() {
        assert_eq!(
            min_valid_strings(
                vec!["abc".into(), "aaaaa".into(), "bcdef".into()],
                "aabcdabc".into()
            ),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_valid_strings(vec!["abababab".into(), "ab".into()], "ababaababa".into()),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(min_valid_strings(vec!["abcdef".into()], "xyz".into()), -1);
    }
}
