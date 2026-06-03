/// LeetCode #1722 - Minimize Hamming Distance After Swap Operations
use std::collections::HashMap;

fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

fn minimize_hamming_distance(
    source: Vec<i32>,
    target: Vec<i32>,
    allowed_swaps: Vec<Vec<i32>>,
) -> i32 {
    let n = source.len();
    let mut parent: Vec<usize> = (0..n).collect();
    for s in allowed_swaps {
        let a = s[0] as usize;
        let b = s[1] as usize;
        let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
        if ra != rb {
            parent[ra] = rb;
        }
    }
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        groups.entry(find(&mut parent, i)).or_default().push(i);
    }
    let mut ans = 0i32;
    for indices in groups.values() {
        let mut cnt_s: HashMap<i32, i32> = HashMap::new();
        let mut cnt_t: HashMap<i32, i32> = HashMap::new();
        for &i in indices {
            *cnt_s.entry(source[i]).or_default() += 1;
            *cnt_t.entry(target[i]).or_default() += 1;
        }
        let matched: i32 = cnt_s
            .iter()
            .map(|(v, &c)| c.min(*cnt_t.get(v).unwrap_or(&0)))
            .sum();
        ans += indices.len() as i32 - matched;
    }
    ans
}
fn main() {
    println!(
        "{}",
        minimize_hamming_distance(
            vec![1, 2, 3, 4],
            vec![2, 1, 4, 5],
            vec![vec![0, 1], vec![2, 3]],
        )
    );
}
#[cfg(test)]
mod tests {
    use super::minimize_hamming_distance;
    #[test]
    fn example_one() {
        assert_eq!(
            minimize_hamming_distance(
                vec![1, 2, 3, 4],
                vec![2, 1, 4, 5],
                vec![vec![0, 1], vec![2, 3]],
            ),
            1
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(
            minimize_hamming_distance(vec![1, 2, 3, 4], vec![1, 3, 2, 4], vec![]),
            2
        );
    }
    #[test]
    fn example_three() {
        assert_eq!(
            minimize_hamming_distance(
                vec![5, 1, 2, 4, 3],
                vec![1, 5, 4, 2, 3],
                vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]],
            ),
            0
        );
    }
}
