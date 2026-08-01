const MOD: i64 = 1_000_000_007;

/// LeetCode #2851 - String Transformation
fn number_of_ways(s: String, t: String, k: i64) -> i32 {
    let n = s.len();
    if n != t.len() || n == 0 {
        return 0;
    }

    fn count_matches(text: &[u8], pattern: &[u8]) -> i64 {
        let mut prefix = vec![0; pattern.len()];
        for i in 1..pattern.len() {
            let mut j = prefix[i - 1];
            while j > 0 && pattern[i] != pattern[j] {
                j = prefix[j - 1];
            }
            if pattern[i] == pattern[j] {
                j += 1;
            }
            prefix[i] = j;
        }

        let mut matches = 0;
        let mut j = 0;
        for &byte in text {
            while j > 0 && byte != pattern[j] {
                j = prefix[j - 1];
            }
            if byte == pattern[j] {
                j += 1;
            }
            if j == pattern.len() {
                matches += 1;
                j = prefix[j - 1];
            }
        }
        matches
    }

    let mut rotations = s.as_bytes().to_vec();
    rotations.extend_from_slice(&s.as_bytes()[..n - 1]);
    let matching_rotations = count_matches(&rotations, t.as_bytes());
    if matching_rotations == 0 {
        return 0;
    }

    type Matrix = [[i64; 2]; 2];
    fn multiply(left: Matrix, right: Matrix) -> Matrix {
        let mut result = [[0; 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                for mid in 0..2 {
                    result[row][col] =
                        (result[row][col] + left[row][mid] * right[mid][col]) % MOD;
                }
            }
        }
        result
    }

    fn power(mut matrix: Matrix, mut exponent: i64) -> Matrix {
        let mut result = [[1, 0], [0, 1]];
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = multiply(result, matrix);
            }
            matrix = multiply(matrix, matrix);
            exponent /= 2;
        }
        result
    }

    let non_matching = n as i64 - matching_rotations;
    let transition = [
        [(matching_rotations - 1).rem_euclid(MOD), matching_rotations],
        [non_matching, (non_matching - 1).rem_euclid(MOD)],
    ];
    let powered = power(transition, k);
    let start_good = i64::from(s == t);
    let start_bad = 1 - start_good;
    ((powered[0][0] * start_good + powered[0][1] * start_bad) % MOD) as i32
}

fn main() {
    println!(
        "{}",
        number_of_ways("abcd".into(), "cdab".into(), 2)
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_ways;

    #[test]
    fn example_one() {
        assert_eq!(number_of_ways("abcd".into(), "cdab".into(), 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_ways("ababab".into(), "ababab".into(), 1), 2);
    }
}
