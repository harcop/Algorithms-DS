/// LeetCode #1287 - Element Appearing More Than 25% In Sorted Array
fn find_special_integer(arr: Vec<i32>) -> i32 {
    let need = arr.len() / 4 + 1;
    let mut count = 1usize;
    for i in 1..arr.len() {
        if arr[i] == arr[i - 1] {
            count += 1;
            if count > need {
                return arr[i];
            }
        } else {
            count = 1;
        }
    }
    arr[0]
}

fn main() {
    println!("{}", find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]));
}

#[cfg(test)]
mod tests {
    use super::find_special_integer;

    #[test]
    fn example_one() {
        assert_eq!(find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_special_integer(vec![1, 1]), 1);
    }
}
