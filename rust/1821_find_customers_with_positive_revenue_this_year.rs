/// LeetCode #1821 - Find Customers With Positive Revenue this Year (SQL; Rust analogue)
fn customers_with_positive_revenue(customers: Vec<(i32, i32, i32)>) -> Vec<i32> {
    let mut ans: Vec<i32> = customers
        .into_iter()
        .filter(|(_, year, revenue)| *year == 2021 && *revenue > 0)
        .map(|(id, _, _)| id)
        .collect();
    ans.sort();
    ans
}

fn main() {
    let customers = vec![
        (1, 2018, 50),
        (1, 2021, 30),
        (1, 2020, 70),
        (2, 2021, -50),
        (3, 2018, 10),
        (3, 2016, 50),
        (4, 2021, 20),
    ];
    println!("{:?}", customers_with_positive_revenue(customers));
}

#[cfg(test)]
mod tests {
    use super::customers_with_positive_revenue;

    #[test]
    fn example_one() {
        let customers = vec![
            (1, 2018, 50),
            (1, 2021, 30),
            (1, 2020, 70),
            (2, 2021, -50),
            (3, 2018, 10),
            (3, 2016, 50),
            (4, 2021, 20),
        ];
        assert_eq!(customers_with_positive_revenue(customers), vec![1, 4]);
    }
}
