/// LeetCode #323 - Number of Connected Components in an Undirected Graph
fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut parent: Vec<i32> = (0..n as i32).collect();
    fn find(p: &mut [i32], mut x: i32) -> i32 {
        while p[x as usize] != x {
            p[x as usize] = p[p[x as usize] as usize];
            x = p[x as usize];
        }
        x
    }
    fn union(p: &mut [i32], a: i32, b: i32) -> bool {
        let ra = find(p, a);
        let rb = find(p, b);
        if ra == rb {
            return false;
        }
        p[ra as usize] = rb;
        true
    }
    let mut comps = n as i32;
    for e in edges {
        if union(&mut parent, e[0], e[1]) {
            comps -= 1;
        }
    }
    comps
}

fn main() {
    println!("{}", count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::count_components;

    #[test]
    fn example() {
        assert_eq!(
            count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]),
            2
        );
    }
}
