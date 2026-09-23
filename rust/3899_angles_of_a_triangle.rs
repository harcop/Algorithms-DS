/// LeetCode #3899 - Angles of a Triangle
fn angles_of_triangle(mut sides: Vec<i32>) -> Vec<f64> {
    sides.sort_unstable();
    let (a, b, c) = (sides[0] as f64, sides[1] as f64, sides[2] as f64);
    if a + b <= c {
        return vec![];
    }
    let angle = |opp: f64, x: f64, y: f64| {
        let cos = (x * x + y * y - opp * opp) / (2.0 * x * y);
        cos.clamp(-1.0, 1.0).acos().to_degrees()
    };
    let mut ans = vec![
        angle(a, b, c),
        angle(b, a, c),
        angle(c, a, b),
    ];
    ans.sort_by(|u, v| u.partial_cmp(v).unwrap());
    ans
}

fn main() {
    println!("{:?}", angles_of_triangle(vec![3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::angles_of_triangle;

    fn approx_eq(got: Vec<f64>, want: &[f64]) {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want.iter()) {
            assert!((g - w).abs() < 1e-4, "got {g} want {w}");
        }
    }

    #[test]
    fn example1() {
        approx_eq(
            angles_of_triangle(vec![3, 4, 5]),
            &[36.86990, 53.13010, 90.0],
        );
    }

    #[test]
    fn example2() {
        assert_eq!(angles_of_triangle(vec![2, 4, 2]), Vec::<f64>::new());
    }
}
