/// LeetCode #212 - Word Search II
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}

fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    if board.is_empty() {
        return vec![];
    }
    let mut root = TrieNode::default();
    for w in words {
        let mut cur = &mut root;
        for c in w.chars() {
            cur = cur.children.entry(c).or_default();
        }
        cur.word = Some(w);
    }
    let m = board.len();
    let n = board[0].len();
    let mut found = HashSet::new();

    fn dfs(
        board: &mut Vec<Vec<char>>,
        node: &mut TrieNode,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
        found: &mut HashSet<String>,
    ) {
        let ch = board[i][j];
        let child = match node.children.get_mut(&ch) {
            Some(c) => c,
            None => return,
        };
        if let Some(w) = child.word.take() {
            found.insert(w);
        }
        board[i][j] = '#';
        if i > 0 && board[i - 1][j] != '#' {
            dfs(board, child, i - 1, j, m, n, found);
        }
        if i + 1 < m && board[i + 1][j] != '#' {
            dfs(board, child, i + 1, j, m, n, found);
        }
        if j > 0 && board[i][j - 1] != '#' {
            dfs(board, child, i, j - 1, m, n, found);
        }
        if j + 1 < n && board[i][j + 1] != '#' {
            dfs(board, child, i, j + 1, m, n, found);
        }
        board[i][j] = ch;
    }

    for i in 0..m {
        for j in 0..n {
            dfs(&mut board, &mut root, i, j, m, n, &mut found);
        }
    }
    let mut out: Vec<String> = found.into_iter().collect();
    out.sort();
    out
}

fn main() {
    println!("{:?}", find_words(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_words;

    #[test]
    fn example_one() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec![
            "oath".into(),
            "pea".into(),
            "eat".into(),
            "rain".into(),
        ];
        let mut got = find_words(board, words);
        got.sort();
        let mut exp: Vec<String> = vec!["eat".into(), "oath".into()];
        exp.sort();
        assert_eq!(got, exp);
    }
}
