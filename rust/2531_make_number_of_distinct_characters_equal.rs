/// LeetCode #2531 - Make Number of Distinct Characters Equal
fn is_it_possible(word1: String, word2: String) -> bool {
    let mut count1 = [0i32; 26];
    let mut count2 = [0i32; 26];
    for c in word1.bytes() {
        count1[(c - b'a') as usize] += 1;
    }
    for c in word2.bytes() {
        count2[(c - b'a') as usize] += 1;
    }
    let distinct1 = count1.iter().filter(|&&c| c > 0).count();
    let distinct2 = count2.iter().filter(|&&c| c > 0).count();

    for i in 0..26 {
        for j in 0..26 {
            if count1[i] == 0 || count2[j] == 0 {
                continue;
            }
            if i == j {
                if distinct1 == distinct2 {
                    return true;
                }
                continue;
            }
            let after1 = distinct1 - (count1[i] == 1) as usize + (count1[j] == 0) as usize;
            let after2 = distinct2 - (count2[j] == 1) as usize + (count2[i] == 0) as usize;
            if after1 == after2 {
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("{}", is_it_possible("ab".to_string(), "b".to_string()));
}

#[cfg(test)]
mod tests {
    use super::is_it_possible;

    #[test]
    fn example_one() {
        assert!(is_it_possible("ab".to_string(), "b".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(is_it_possible("abcd".to_string(), "bcdf".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(!is_it_possible(
            "aaaaaaabc".to_string(),
            "wwwwwwwwww".to_string()
        ));
    }
}
