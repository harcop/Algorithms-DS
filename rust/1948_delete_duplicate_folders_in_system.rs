/// LeetCode #1948 - Delete Duplicate Folders in System
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<String, usize>,
}

fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut nodes = vec![TrieNode {
        children: HashMap::new(),
    }];

    for path in paths {
        let mut cur = 0usize;
        for name in path {
            let next = if let Some(&idx) = nodes[cur].children.get(&name) {
                idx
            } else {
                let idx = nodes.len();
                nodes.push(TrieNode {
                    children: HashMap::new(),
                });
                nodes[cur].children.insert(name, idx);
                idx
            };
            cur = next;
        }
    }

    let mut deleted = vec![false; nodes.len()];
    let mut seen: HashMap<String, usize> = HashMap::new();

    fn dfs(
        u: usize,
        nodes: &[TrieNode],
        seen: &mut HashMap<String, usize>,
        deleted: &mut [bool],
    ) -> String {
        if nodes[u].children.is_empty() {
            return String::new();
        }
        let mut subs = Vec::new();
        for (name, &v) in &nodes[u].children {
            subs.push(format!("{name}({})", dfs(v, nodes, seen, deleted)));
        }
        subs.sort_unstable();
        let sig = subs.join("");
        if let Some(&other) = seen.get(&sig) {
            deleted[other] = true;
            deleted[u] = true;
        } else {
            seen.insert(sig.clone(), u);
        }
        sig
    }

    fn collect(
        u: usize,
        nodes: &[TrieNode],
        deleted: &[bool],
        path: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if deleted[u] {
            return;
        }
        if !path.is_empty() {
            ans.push(path.clone());
        }
        let mut names: Vec<_> = nodes[u].children.keys().cloned().collect();
        names.sort_unstable();
        for name in names {
            let v = nodes[u].children[&name];
            path.push(name);
            collect(v, nodes, deleted, path, ans);
            path.pop();
        }
    }

    dfs(0, &nodes, &mut seen, &mut deleted);
    let mut ans = Vec::new();
    let mut path = Vec::new();
    collect(0, &nodes, &deleted, &mut path, &mut ans);
    ans
}

fn main() {
    let paths = vec![
        vec!["a".into()],
        vec!["c".into()],
        vec!["d".into()],
        vec!["a".into(), "b".into()],
        vec!["c".into(), "b".into()],
        vec!["d".into(), "a".into()],
    ];
    println!("{:?}", delete_duplicate_folder(paths));
}

#[cfg(test)]
mod tests {
    use super::delete_duplicate_folder;

    fn sort_paths(mut paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        paths.sort();
        paths
    }

    #[test]
    fn example_one() {
        let paths = vec![
            vec!["a".into()],
            vec!["c".into()],
            vec!["d".into()],
            vec!["a".into(), "b".into()],
            vec!["c".into(), "b".into()],
            vec!["d".into(), "a".into()],
        ];
        assert_eq!(
            sort_paths(delete_duplicate_folder(paths)),
            sort_paths(vec![vec!["d".into()], vec!["d".into(), "a".into()]])
        );
    }

    #[test]
    fn example_two() {
        let paths = vec![
            vec!["a".into()],
            vec!["c".into()],
            vec!["a".into(), "b".into()],
            vec!["c".into(), "b".into()],
            vec!["a".into(), "b".into(), "x".into()],
            vec!["a".into(), "b".into(), "x".into(), "y".into()],
            vec!["w".into()],
            vec!["w".into(), "y".into()],
        ];
        assert_eq!(
            sort_paths(delete_duplicate_folder(paths)),
            sort_paths(vec![
                vec!["a".into()],
                vec!["a".into(), "b".into()],
                vec!["c".into()],
                vec!["c".into(), "b".into()],
            ])
        );
    }
}
