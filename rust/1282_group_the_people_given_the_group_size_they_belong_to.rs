/// LeetCode #1282 - Group the People Given the Group Size They Belong To
fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    let mut buckets: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    let mut res = Vec::new();
    for (i, &gs) in group_sizes.iter().enumerate() {
        let b = buckets.entry(gs).or_default();
        b.push(i as i32);
        if b.len() == gs as usize {
            res.push(b.clone());
            b.clear();
        }
    }
    res
}

fn main() {
    println!("{:?}", group_the_people(vec![3, 3, 3, 3, 3, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::group_the_people;

    fn normalize(mut groups: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for g in &mut groups {
            g.sort_unstable();
        }
        groups.sort_unstable();
        groups
    }

    #[test]
    fn example_one() {
        let got = normalize(group_the_people(vec![3, 3, 3, 3, 3, 1, 3]));
        let want = normalize(vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]);
        assert_eq!(got, want);
    }

    #[test]
    fn example_two() {
        let got = normalize(group_the_people(vec![2, 1, 3, 3, 3, 2]));
        let want = normalize(vec![vec![1], vec![0, 5], vec![2, 3, 4]]);
        assert_eq!(got, want);
    }
}
