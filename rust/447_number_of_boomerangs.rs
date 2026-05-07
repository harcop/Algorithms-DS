/// LeetCode #447 - Number of Boomerangs
fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut ans = 0;
    for i in 0..n {
        use std::collections::HashMap;
        let mut m: HashMap<i32, i32> = HashMap::new();
        for j in 0..n {
            if i == j {
                continue;
            }
            let dx = points[i][0] - points[j][0];
            let dy = points[i][1] - points[j][1];
            let d = dx * dx + dy * dy;
            *m.entry(d).or_insert(0) += 1;
        }
        for &c in m.values() {
            if c >= 2 {
                ans += c * (c - 1);
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_boomerangs;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]),
            2
        );
    }
}
