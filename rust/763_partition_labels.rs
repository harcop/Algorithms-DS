/// LeetCode #763 - Partition Labels
fn partition_labels(s: String) -> Vec<i32> {
    let mut last = [0usize; 26];
    for (i, c) in s.bytes().enumerate() {
        last[(c - b'a') as usize] = i;
    }
    let mut ans = vec![];
    let mut anchor = 0usize;
    let mut j = 0usize;
    for (i, c) in s.bytes().enumerate() {
        j = j.max(last[(c - b'a') as usize]);
        if i == j {
            ans.push((i - anchor + 1) as i32);
            anchor = i + 1;
        }
    }
    ans
}

fn main() {
    println!("{:?}", partition_labels("abc".into()));
}

#[cfg(test)]
mod tests {
    use super::partition_labels;

    #[test]
    fn partitions_each_unique_char() {
        assert_eq!(partition_labels("abc".into()), vec![1, 1, 1]);
    }
}
