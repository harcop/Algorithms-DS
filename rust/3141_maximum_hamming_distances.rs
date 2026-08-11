/// LeetCode #3141 - Maximum Hamming Distances
fn max_hamming_distances(nums: Vec<i32>, m: i32) -> Vec<i32> {
    let m = m as usize;
    let size = 1 << m;
    let mut dist = vec![-1i32; size];
    let mut q: Vec<usize> = Vec::new();
    for &x in &nums {
        let x = x as usize;
        dist[x] = 0;
        q.push(x);
    }
    let mut k = 1;
    while !q.is_empty() {
        let mut t = Vec::new();
        for &x in &q {
            for i in 0..m {
                let y = x ^ (1 << i);
                if dist[y] == -1 {
                    t.push(y);
                    dist[y] = k;
                }
            }
        }
        q = t;
        k += 1;
    }
    let mask = size - 1;
    nums.into_iter()
        .map(|x| m as i32 - dist[(x as usize) ^ mask])
        .collect()
}

fn main() {
    println!("{:?}", max_hamming_distances(vec![9, 12, 9, 11], 4));
}

#[cfg(test)]
mod tests {
    use super::max_hamming_distances;

    #[test]
    fn example1() {
        assert_eq!(
            max_hamming_distances(vec![9, 12, 9, 11], 4),
            vec![2, 3, 2, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_hamming_distances(vec![3, 4, 6, 10], 4),
            vec![3, 3, 2, 3]
        );
    }
}
