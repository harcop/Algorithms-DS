/// LeetCode #1899 - Merge Triplets to Form Target Triplet
fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let (x, y, z) = (target[0], target[1], target[2]);
    let (mut d, mut e, mut f) = (0, 0, 0);
    for t in triplets {
        let (a, b, c) = (t[0], t[1], t[2]);
        if a <= x && b <= y && c <= z {
            d = d.max(a);
            e = e.max(b);
            f = f.max(c);
        }
    }
    d == x && e == y && f == z
}

fn main() {
    println!(
        "{}",
        merge_triplets(
            vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]],
            vec![2, 7, 5]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::merge_triplets;

    #[test]
    fn example_one() {
        assert!(merge_triplets(
            vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]],
            vec![2, 7, 5]
        ));
    }

    #[test]
    fn example_two() {
        assert!(!merge_triplets(
            vec![vec![3, 4, 5], vec![4, 5, 6]],
            vec![3, 2, 5]
        ));
    }
}
