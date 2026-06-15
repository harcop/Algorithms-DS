/// LeetCode #1868 - Product of Two Run-Length Encoded Arrays
fn find_rle_array(encoded1: Vec<Vec<i32>>, mut encoded2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut j = 0usize;
    for pair in encoded1 {
        let vi = pair[0];
        let mut fi = pair[1];
        while fi > 0 {
            let f = fi.min(encoded2[j][1]);
            let v = vi * encoded2[j][0];
            if let Some(last) = ans.last_mut() {
                if last[0] == v {
                    last[1] += f;
                } else {
                    ans.push(vec![v, f]);
                }
            } else {
                ans.push(vec![v, f]);
            }
            fi -= f;
            encoded2[j][1] -= f;
            if encoded2[j][1] == 0 {
                j += 1;
            }
        }
    }
    ans
}

fn main() {
    let encoded1 = vec![vec![1, 3], vec![2, 3]];
    let encoded2 = vec![vec![6, 3], vec![3, 3]];
    println!("{:?}", find_rle_array(encoded1, encoded2));
}

#[cfg(test)]
mod tests {
    use super::find_rle_array;

    #[test]
    fn example_one() {
        assert_eq!(
            find_rle_array(vec![vec![1, 3], vec![2, 3]], vec![vec![6, 3], vec![3, 3]]),
            vec![vec![6, 6]]
        );
    }
}
