/// LeetCode #1835 - Find XOR Sum of All Pairs Bitwise AND
fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let x1 = arr1.iter().fold(0, |acc, &v| acc ^ v);
    let x2 = arr2.iter().fold(0, |acc, &v| acc ^ v);
    x1 & x2
}

fn main() {
    println!("{}", get_xor_sum(vec![1, 2, 3], vec![6, 5]));
}

#[cfg(test)]
mod tests {
    use super::get_xor_sum;

    #[test]
    fn example_one() {
        assert_eq!(get_xor_sum(vec![1, 2, 3], vec![6, 5]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_xor_sum(vec![12], vec![4]), 4);
    }
}
