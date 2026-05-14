/// LeetCode #739 - Daily Temperatures
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut ans = vec![0i32; n];
    let mut st: Vec<usize> = vec![];
    for i in 0..n {
        while let Some(&j) = st.last() {
            if temperatures[i] > temperatures[j] {
                st.pop();
                ans[j] = (i - j) as i32;
            } else {
                break;
            }
        }
        st.push(i);
    }
    ans
}

fn main() {
    println!("{:?}", daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]));
}

#[cfg(test)]
mod tests {
    use super::daily_temperatures;

    #[test]
    fn example_one() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
