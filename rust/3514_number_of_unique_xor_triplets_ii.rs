/// LeetCode #3514 - Number of Unique XOR Triplets II
fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let mx = (*nums.iter().max().unwrap() as usize) << 1;
    let mut st = vec![false; mx];
    for &a in &nums {
        for &b in &nums {
            st[(a ^ b) as usize] = true;
        }
    }
    let mut s = vec![0i32; mx];
    for (ab, &ok) in st.iter().enumerate() {
        if ok {
            for &c in &nums {
                s[ab ^ c as usize] = 1;
            }
        }
    }
    s.iter().sum()
}

fn main() {
    println!("{}", unique_xor_triplets(vec![1, 3]));
}

#[cfg(test)]
mod tests {
    use super::unique_xor_triplets;

    #[test]
    fn example1() {
        assert_eq!(unique_xor_triplets(vec![1, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(unique_xor_triplets(vec![6, 7, 8, 9]), 4);
    }
}
