/// LeetCode #2445 - Number of Nodes With Value One
fn number_of_nodes(n: i32, queries: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut flipped = vec![false; n + 1];
    for query in queries {
        flipped[query as usize] ^= true;
    }

    let mut value = vec![false; n + 1];
    let mut answer = 0;
    for node in 1..=n {
        value[node] = value[node / 2] ^ flipped[node];
        if value[node] {
            answer += 1;
        }
    }

    answer
}

fn main() {
    println!("{}", number_of_nodes(5, vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::number_of_nodes;

    #[test]
    fn flips_overlapping_subtrees() {
        assert_eq!(number_of_nodes(5, vec![1, 2]), 2);
    }

    #[test]
    fn duplicate_queries_cancel() {
        assert_eq!(number_of_nodes(4, vec![2, 2]), 0);
    }
}
