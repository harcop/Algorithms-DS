/// LeetCode #2517 - Maximum Tastiness of Candy Basket
fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
    price.sort_unstable();
    let mut l = 0;
    let mut r = price[price.len() - 1] - price[0];
    while l < r {
        let mid = (l + r + 1) >> 1;
        if check(&price, k, mid) {
            l = mid;
        } else {
            r = mid - 1;
        }
    }
    l
}

fn check(price: &[i32], k: i32, x: i32) -> bool {
    let mut cnt = 0;
    let mut pre = -x;
    for &cur in price {
        if cur - pre >= x {
            pre = cur;
            cnt += 1;
        }
    }
    cnt >= k
}

fn main() {
    println!("{}", maximum_tastiness(vec![13, 5, 1, 8, 21, 2], 3));
}

#[cfg(test)]
mod tests {
    use super::maximum_tastiness;

    #[test]
    fn example_one() {
        assert_eq!(maximum_tastiness(vec![13, 5, 1, 8, 21, 2], 3), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_tastiness(vec![1, 3, 1], 2), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_tastiness(vec![7, 7, 7, 7], 2), 0);
    }
}
