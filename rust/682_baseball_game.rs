/// LeetCode #682 - Baseball Game
fn cal_points(operations: Vec<String>) -> i32 {
    let mut st: Vec<i32> = vec![];
    for op in operations {
        match op.as_str() {
            "+" => {
                let n = st.len();
                let v = st[n - 1] + st[n - 2];
                st.push(v);
            }
            "D" => {
                let v = *st.last().unwrap() * 2;
                st.push(v);
            }
            "C" => { st.pop(); }
            x => st.push(x.parse::<i32>().unwrap()),
        }
    }
    st.iter().sum()
}

fn main() {
    println!("{}", cal_points(vec!["5".into(),"2".into(),"C".into(),"D".into(),"+".into()]));
}

#[cfg(test)]
mod tests {
    use super::cal_points;

    #[test]
    fn example_one() {
        assert_eq!(cal_points(vec!["5".into(),"2".into(),"C".into(),"D".into(),"+".into()]), 30);
    }
}
