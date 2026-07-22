/// LeetCode #2564 - Substring XOR Queries
use std::collections::HashMap;

fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut d: HashMap<i32, [i32; 2]> = HashMap::new();

    for i in 0..n {
        let mut x = 0i32;
        for j in 0..32 {
            if i + j >= n {
                break;
            }
            x = (x << 1) | (bytes[i + j] - b'0') as i32;
            d.entry(x).or_insert([i as i32, (i + j) as i32]);
            if x == 0 {
                break;
            }
        }
    }

    queries
        .into_iter()
        .map(|q| {
            let val = q[0] ^ q[1];
            d.get(&val)
                .map(|p| vec![p[0], p[1]])
                .unwrap_or_else(|| vec![-1, -1])
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        substring_xor_queries("101101".to_string(), vec![vec![0, 5], vec![1, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::substring_xor_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            substring_xor_queries("101101".to_string(), vec![vec![0, 5], vec![1, 2]]),
            vec![vec![0, 2], vec![2, 3]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            substring_xor_queries("0101".to_string(), vec![vec![12, 8]]),
            vec![vec![-1, -1]]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            substring_xor_queries("1".to_string(), vec![vec![4, 5]]),
            vec![vec![0, 0]]
        );
    }
}
