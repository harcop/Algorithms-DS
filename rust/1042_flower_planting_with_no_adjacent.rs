/// LeetCode #1042 - Flower Planting With No Adjacent
fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n + 1];
    for p in paths {
        let a = p[0] as usize;
        let b = p[1] as usize;
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut ans = vec![0i32; n + 1];
    for i in 1..=n {
        let mut used = [false; 5];
        for &nb in &adj[i] {
            used[ans[nb] as usize] = true;
        }
        for c in 1..=4 {
            if !used[c] {
                ans[i] = c as i32;
                break;
            }
        }
    }
    ans[1..].to_vec()
}

fn main() {
    println!("{:?}", garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]));
}

#[cfg(test)]
mod tests {
    use super::garden_no_adj;

    #[test]
    fn example_one() {
        assert_eq!(garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]), vec![1, 2, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(garden_no_adj(4, vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 1, 2]);
    }
}
