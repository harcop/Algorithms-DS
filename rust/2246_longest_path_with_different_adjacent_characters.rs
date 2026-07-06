/// LeetCode #2246 - Longest Path With Different Adjacent Characters
fn longest_path(parent: Vec<i32>, s: String) -> i32 {
    let n = parent.len();
    let chars: Vec<char> = s.chars().collect();
    let mut graph = vec![vec![]; n];

    for i in 1..n {
        graph[parent[i] as usize].push(i);
    }

    let mut ans = 1;
    dfs(0, &graph, &chars, &mut ans);
    ans
}

fn dfs(u: usize, graph: &[Vec<usize>], s: &[char], ans: &mut i32) -> i32 {
    let mut max1 = 0;
    let mut max2 = 0;

    for &v in &graph[u] {
        let res = dfs(v, graph, s, ans);
        if s[u] == s[v] {
            continue;
        }
        if res > max1 {
            max2 = max1;
            max1 = res;
        } else if res > max2 {
            max2 = res;
        }
    }

    *ans = (*ans).max(1 + max1 + max2);
    1 + max1
}

fn main() {
    println!(
        "{}",
        longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::longest_path;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string()),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_path(vec![-1, 0, 0, 0], "aabc".to_string()), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string()), 3);
    }
}
