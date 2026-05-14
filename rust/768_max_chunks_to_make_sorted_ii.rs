/// LeetCode #768 - Max Chunks To Make Sorted II
fn max_chunks_to_sorted_ii(arr: Vec<i32>) -> i32 {
    let mut st: Vec<i32> = Vec::new();
    for x in arr {
        let mut mx = x;
        while let Some(&y) = st.last() {
            if y > x {
                mx = mx.max(st.pop().unwrap());
            } else {
                break;
            }
        }
        if st.last() != Some(&mx) {
            st.push(mx);
        }
    }
    st.len() as i32
}

fn main() {
    println!("{}", max_chunks_to_sorted_ii(vec![2, 1, 3, 4, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_chunks_to_sorted_ii;

    #[test]
    fn example_one() {
        assert_eq!(max_chunks_to_sorted_ii(vec![5, 4, 3, 2, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_chunks_to_sorted_ii(vec![2, 1, 3, 4, 4]), 3);
    }
}
