/// LeetCode #3053 - Classifying Triangles by Lengths (SQL; Rust analogue)

fn classify_triangles(sides: Vec<(i32, i32, i32)>) -> Vec<String> {
    sides
        .into_iter()
        .map(|(a, b, c)| {
            if a + b <= c || a + c <= b || b + c <= a {
                "Not A Triangle".into()
            } else if a == b && b == c {
                "Equilateral".into()
            } else if a == b || a == c || b == c {
                "Isosceles".into()
            } else {
                "Scalene".into()
            }
        })
        .collect()
}

fn main() {
    let sides = vec![(20, 20, 23), (20, 20, 20), (20, 21, 22), (13, 14, 30)];
    println!("{:?}", classify_triangles(sides));
}

#[cfg(test)]
mod tests {
    use super::classify_triangles;

    #[test]
    fn example() {
        let sides = vec![(20, 20, 23), (20, 20, 20), (20, 21, 22), (13, 14, 30)];
        assert_eq!(
            classify_triangles(sides),
            vec![
                "Isosceles".to_string(),
                "Equilateral".to_string(),
                "Scalene".to_string(),
                "Not A Triangle".to_string(),
            ]
        );
    }
}
