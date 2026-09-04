/// LeetCode #610 - Triangle Judgement (SQL; Rust analogue)
fn triangle_judgement(triangle: Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32, String)> {
    triangle
        .into_iter()
        .map(|(x, y, z)| {
            let ok = x + y > z && x + z > y && y + z > x;
            (x, y, z, if ok { "Yes" } else { "No" }.to_string())
        })
        .collect()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::triangle_judgement;

    #[test]
    fn example() {
        let triangle = vec![(13, 15, 30), (10, 20, 15)];
        assert_eq!(
            triangle_judgement(triangle),
            vec![
                (13, 15, 30, "No".into()),
                (10, 20, 15, "Yes".into()),
            ]
        );
    }
}
