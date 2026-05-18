/// LeetCode #907 - Sum of Subarray Minimums
const MOD: i64 = 1_000_000_007;

fn sum_subarray_mins(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let a: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    let mut left = vec![0i32; n];
    let mut right = vec![0i32; n];
    let mut st: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&j) = st.last() {
            if a[j] > a[i] {
                st.pop();
            } else {
                break;
            }
        }
        left[i] = if let Some(&j) = st.last() {
            i as i32 - j as i32
        } else {
            i as i32 + 1
        };
        st.push(i);
    }
    st.clear();
    for i in (0..n).rev() {
        while let Some(&j) = st.last() {
            if a[j] >= a[i] {
                st.pop();
            } else {
                break;
            }
        }
        right[i] = if let Some(&j) = st.last() {
            j as i32 - i as i32
        } else {
            n as i32 - i as i32
        };
        st.push(i);
    }

    let mut ans: i64 = 0;
    for i in 0..n {
        let contrib = a[i] * left[i] as i64 * right[i] as i64;
        ans = (ans + contrib) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", sum_subarray_mins(vec![3, 1, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::sum_subarray_mins;

    #[test]
    fn example_one() {
        assert_eq!(sum_subarray_mins(vec![3, 1, 2, 4]), 17);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }
}
