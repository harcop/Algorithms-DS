/// LeetCode #2125 - Number of Laser Beams in a Bank
fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut prev = 0;
    let mut ans = 0;

    for row in bank {
        let devices = row.bytes().filter(|&b| b == b'1').count() as i32;
        if devices > 0 {
            ans += prev * devices;
            prev = devices;
        }
    }

    ans
}

fn main() {
    println!(
        "{}",
        number_of_beams(vec![
            "011001".into(),
            "000000".into(),
            "010100".into(),
            "001000".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_beams;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_beams(vec![
                "011001".into(),
                "000000".into(),
                "010100".into(),
                "001000".into()
            ]),
            8
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            number_of_beams(vec!["000".into(), "111".into(), "000".into()]),
            0
        );
    }
}
