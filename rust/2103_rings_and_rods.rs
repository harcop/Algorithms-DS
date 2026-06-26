/// LeetCode #2103 - Rings and Rods
fn count_points(rings: String) -> i32 {
    let mut rods = [0u8; 10];
    let bytes = rings.as_bytes();
    for i in (0..bytes.len()).step_by(2) {
        let color = match bytes[i] {
            b'R' => 1,
            b'G' => 2,
            b'B' => 4,
            _ => 0,
        };
        rods[(bytes[i + 1] - b'0') as usize] |= color;
    }
    rods.iter().filter(|&&mask| mask == 7).count() as i32
}

fn main() {
    println!("{}", count_points("B0B6G0R6R0R6G9".into()));
}

#[cfg(test)]
mod tests {
    use super::count_points;

    #[test]
    fn example_one() {
        assert_eq!(count_points("B0B6G0R6R0R6G9".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_points("B0R0G0R9R0B0G0".into()), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_points("G4".into()), 0);
    }
}
