/// LeetCode #2861 - Maximum Number of Alloys
fn max_number_of_alloys(
    n: i32,
    _k: i32,
    budget: i32,
    composition: Vec<Vec<i32>>,
    stock: Vec<i32>,
    cost: Vec<i32>,
) -> i32 {
    fn can_make(
        amount: i64,
        recipe: &[i32],
        stock: &[i32],
        cost: &[i32],
        budget: i64,
    ) -> bool {
        let mut total_cost = 0i64;
        for index in 0..recipe.len() {
            let required = amount * recipe[index] as i64;
            let missing = (required - stock[index] as i64).max(0);
            total_cost += missing * cost[index] as i64;
            if total_cost > budget {
                return false;
            }
        }
        true
    }

    let mut answer = 0i64;
    for recipe in composition {
        let mut low = 0i64;
        let mut high = 1i64;
        while can_make(high, &recipe, &stock, &cost, budget as i64) {
            low = high;
            high *= 2;
        }

        while low + 1 < high {
            let middle = low + (high - low) / 2;
            if can_make(middle, &recipe, &stock, &cost, budget as i64) {
                low = middle;
            } else {
                high = middle;
            }
        }
        answer = answer.max(low);
    }

    debug_assert_eq!(stock.len(), n as usize);
    answer as i32
}

fn main() {
    println!(
        "{}",
        max_number_of_alloys(
            3,
            2,
            15,
            vec![vec![1, 1, 1], vec![1, 1, 10]],
            vec![0, 0, 0],
            vec![1, 2, 3],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_number_of_alloys;

    #[test]
    fn example_one() {
        assert_eq!(
            max_number_of_alloys(
                3,
                2,
                15,
                vec![vec![1, 1, 1], vec![1, 1, 10]],
                vec![0, 0, 0],
                vec![1, 2, 3],
            ),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_number_of_alloys(
                3,
                2,
                15,
                vec![vec![1, 1, 1], vec![1, 1, 10]],
                vec![0, 0, 100],
                vec![1, 2, 3],
            ),
            5
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            max_number_of_alloys(
                2,
                3,
                10,
                vec![vec![2, 1], vec![1, 2], vec![1, 1]],
                vec![1, 1],
                vec![5, 5],
            ),
            2
        );
    }
}
