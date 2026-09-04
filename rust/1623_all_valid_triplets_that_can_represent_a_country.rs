/// LeetCode #1623 - All Valid Triplets That Can Represent a Country (SQL; Rust analogue)
fn valid_triplets(
    school_a: Vec<(i32, String)>,
    school_b: Vec<(i32, String)>,
    school_c: Vec<(i32, String)>,
) -> Vec<(String, String, String)> {
    let mut ans = Vec::new();
    for (id_a, name_a) in &school_a {
        for (id_b, name_b) in &school_b {
            for (id_c, name_c) in &school_c {
                if id_a != id_b
                    && id_a != id_c
                    && id_b != id_c
                    && name_a != name_b
                    && name_a != name_c
                    && name_b != name_c
                {
                    ans.push((name_a.clone(), name_b.clone(), name_c.clone()));
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{:?}", valid_triplets(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::valid_triplets;

    #[test]
    fn example() {
        let a = vec![(1, "Alice".into()), (2, "Bob".into())];
        let b = vec![(3, "Tom".into())];
        let c = vec![
            (3, "Tom".into()),
            (2, "Jerry".into()),
            (10, "Alice".into()),
        ];
        let mut got = valid_triplets(a, b, c);
        got.sort();
        assert_eq!(
            got,
            vec![
                ("Alice".into(), "Tom".into(), "Jerry".into()),
                ("Bob".into(), "Tom".into(), "Alice".into()),
            ]
        );
    }
}
