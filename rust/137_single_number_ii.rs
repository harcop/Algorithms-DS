/// LeetCode #137 - Single Number II
fn single_number(nums: Vec<i32>) -> i32 {
    let mut ones = 0i32;
    let mut twos = 0i32;
    for n in nums {
        twos |= ones & n;
        ones ^= n;
        let threes = ones & twos;
        ones &= !threes;
        twos &= !threes;
    }
    ones
}

fn main() {
    println!("{}", single_number(vec![0, 1, 0, 1, 0, 1, 99]));
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn example_one() {
        assert_eq!(single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
