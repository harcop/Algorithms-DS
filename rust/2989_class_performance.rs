/// LeetCode #2989 - Class Performance (SQL; Rust analogue)
fn difference_in_score(scores: Vec<(i32, String, i32, i32, i32)>) -> i32 {
    // (student_id, name, a1, a2, a3)
    let totals: Vec<i32> = scores
        .into_iter()
        .map(|(_, _, a, b, c)| a + b + c)
        .collect();
    totals.iter().copied().max().unwrap() - totals.iter().copied().min().unwrap()
}

fn main() {
    let scores = vec![
        (309, "Owen".into(), 88, 47, 87),
        (321, "Claire".into(), 98, 95, 37),
        (338, "Julian".into(), 100, 64, 43),
        (423, "Peyton".into(), 60, 44, 47),
        (896, "David".into(), 32, 37, 50),
        (235, "Camila".into(), 31, 53, 69),
    ];
    println!("{}", difference_in_score(scores));
}

#[cfg(test)]
mod tests {
    use super::difference_in_score;

    #[test]
    fn example() {
        let scores = vec![
            (309, "Owen".into(), 88, 47, 87),
            (321, "Claire".into(), 98, 95, 37),
            (338, "Julian".into(), 100, 64, 43),
            (423, "Peyton".into(), 60, 44, 47),
            (896, "David".into(), 32, 37, 50),
            (235, "Camila".into(), 31, 53, 69),
        ];
        assert_eq!(difference_in_score(scores), 111);
    }
}
