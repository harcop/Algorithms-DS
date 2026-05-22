/// LeetCode #1196 - How Many Apples Can You Put into the Basket
fn max_number_of_apples(weight: Vec<i32>) -> i32 {
    let mut w = weight;
    w.sort_unstable();
    let mut baskets = [0i32; 2];
    let mut count = 0i32;
    for x in w {
        let i = if baskets[0] <= baskets[1] { 0 } else { 1 };
        if baskets[i] + x <= 5000 {
            baskets[i] += x;
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", max_number_of_apples(vec![100, 200, 150, 1000]));
}

#[cfg(test)]
mod tests {
    use super::max_number_of_apples;

    #[test]
    fn example_one() {
        assert_eq!(max_number_of_apples(vec![100, 200, 150, 1000]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_number_of_apples(vec![900, 495, 600]), 3);
    }
}
