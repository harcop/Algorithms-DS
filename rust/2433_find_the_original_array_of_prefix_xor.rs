/// LeetCode #2433 - Find The Original Array of Prefix Xor
fn find_array(pref: Vec<i32>) -> Vec<i32> {
    let mut answer = Vec::with_capacity(pref.len());
    answer.push(pref[0]);

    for i in 1..pref.len() {
        answer.push(pref[i - 1] ^ pref[i]);
    }

    answer
}

fn main() {
    println!("{:?}", find_array(vec![5, 2, 0, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_array;

    #[test]
    fn example_one() {
        assert_eq!(find_array(vec![5, 2, 0, 3, 1]), vec![5, 7, 2, 3, 2]);
    }

    #[test]
    fn single_value() {
        assert_eq!(find_array(vec![13]), vec![13]);
    }
}
