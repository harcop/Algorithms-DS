/// LeetCode #620 - Not Boring Movies (SQL; Rust analogue)
fn not_boring_movies(mut cinema: Vec<(i32, String, String, f64)>) -> Vec<(i32, String, String, f64)> {
    cinema.retain(|(id, _, desc, _)| id % 2 == 1 && desc != "boring");
    cinema.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap());
    cinema
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::not_boring_movies;

    #[test]
    fn example() {
        let cinema = vec![
            (1, "War".into(), "great 3D".into(), 8.9),
            (2, "Science".into(), "fiction".into(), 8.5),
            (3, "irish".into(), "boring".into(), 6.2),
            (4, "Ice song".into(), "Fantacy".into(), 8.6),
            (5, "House card".into(), "Interesting".into(), 9.1),
        ];
        assert_eq!(
            not_boring_movies(cinema),
            vec![
                (5, "House card".into(), "Interesting".into(), 9.1),
                (1, "War".into(), "great 3D".into(), 8.9),
            ]
        );
    }
}
