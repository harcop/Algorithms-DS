/// LeetCode #1557 - Minimum Number Of Vertices To Reach All Nodes
fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut has_in = vec![false; n as usize];
    for e in edges {
        has_in[e[1] as usize] = true;
    }
    (0..n).filter(|&i| !has_in[i as usize]).collect()
}

fn main() {
    println!("{:?}", find_smallest_set_of_vertices(6, vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]]));
}

#[cfg(test)]
mod tests {
    use super::find_smallest_set_of_vertices;

    #[test]
    fn example_one() {
        assert_eq!(find_smallest_set_of_vertices(6, vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]]), vec![0, 3]);
    }
}
