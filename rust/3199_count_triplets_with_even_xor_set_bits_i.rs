/// LeetCode #3199 - Count Triplets with Even XOR Set Bits I
fn triplet_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> i32 {
    let cnt = |xs: &[i32]| -> [i32; 2] {
        let mut c = [0i32; 2];
        for &x in xs {
            c[(x.count_ones() & 1) as usize] += 1;
        }
        c
    };
    let c1 = cnt(&a);
    let c2 = cnt(&b);
    let c3 = cnt(&c);
    let mut ans = 0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                if (i + j + k) % 2 == 0 {
                    ans += c1[i] * c2[j] * c3[k];
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{}", triplet_count(vec![1], vec![2], vec![3]));
}

#[cfg(test)]
mod tests {
    use super::triplet_count;

    #[test]
    fn example1() {
        assert_eq!(triplet_count(vec![1], vec![2], vec![3]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(triplet_count(vec![1, 1], vec![2, 3], vec![1, 5]), 4);
    }
}
