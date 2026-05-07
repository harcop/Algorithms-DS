/// LeetCode #406 - Queue Reconstruction by Height (sort + insert kneed)
fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    people.sort_unstable_by(|a, b| b[0].cmp(&a[0]).then_with(|| a[1].cmp(&b[1])));
    let mut out: Vec<Vec<i32>> = Vec::with_capacity(people.len());
    for p in people {
        out.insert(p[1] as usize, p);
    }
    out
}

fn main() {
    println!(
        "{:?}",
        reconstruct_queue(vec![vec![7, 0], vec![4, 4], vec![7, 1], vec![5, 0], vec![6, 1], vec![5, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        let ans = reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2],
        ]);
        assert_eq!(
            ans,
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1],
            ]
        );
    }
}
