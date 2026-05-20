/// LeetCode #1170 - Compare Strings by Frequency of the Smallest Character
fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    fn f(s: &str) -> i32 {
        let mut min_c = 'z';
        let mut cnt = 0i32;
        for ch in s.chars() {
            if ch < min_c {
                min_c = ch;
                cnt = 1;
            } else if ch == min_c {
                cnt += 1;
            }
        }
        cnt
    }
    let mut wf: Vec<i32> = words.iter().map(|w| f(w)).collect();
    wf.sort_unstable();
    queries
        .iter()
        .map(|q| {
            let fq = f(q);
            (wf.len() - wf.partition_point(|&w| w <= fq)) as i32
        })
        .collect()
}

fn main() {
    let q = vec!["cbd".to_string()];
    let w = vec!["zaaaz".to_string()];
    println!("{:?}", num_smaller_by_frequency(q, w));
}

#[cfg(test)]
mod tests {
    use super::num_smaller_by_frequency;

    #[test]
    fn example_one() {
        assert_eq!(
            num_smaller_by_frequency(vec!["cbd".to_string()], vec!["zaaaz".to_string()]),
            vec![1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            num_smaller_by_frequency(
                vec!["bbb", "cc"].iter().map(|s| s.to_string()).collect(),
                vec!["a", "aa", "aaa", "aaaa"].iter().map(|s| s.to_string()).collect(),
            ),
            vec![1, 2]
        );
    }
}
