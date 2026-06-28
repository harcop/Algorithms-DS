/// LeetCode #2147 - Number of Ways to Divide a Long Corridor
fn number_of_ways(corridor: &str) -> i32 {
    const MOD: i64 = 1_000_000_007;

    let seats: Vec<usize> = corridor
        .bytes()
        .enumerate()
        .filter(|&(_, b)| b == b'S')
        .map(|(i, _)| i)
        .collect();

    if seats.len() % 2 != 0 || seats.is_empty() {
        return 0;
    }

    let mut answer = 1i64;
    for i in (1..seats.len()).step_by(2) {
        if i + 1 < seats.len() {
            answer = answer * (seats[i + 1] - seats[i]) as i64 % MOD;
        }
    }

    answer as i32
}

fn main() {
    println!("{}", number_of_ways("SSPPSPS"));
}

#[cfg(test)]
mod tests {
    use super::number_of_ways;

    #[test]
    fn example_one() {
        assert_eq!(number_of_ways("SSPPSPS"), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_ways("PPSPSP"), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_ways("S"), 0);
    }
}
