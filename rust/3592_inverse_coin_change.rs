/// LeetCode #3592 - Inverse Coin Change
fn find_coins(num_ways: Vec<i32>) -> Vec<i32> {
    let n = num_ways.len();
    let mut f = vec![0i64; n + 1];
    f[0] = 1;
    let mut coins = Vec::new();
    for i in 1..=n {
        let expected = num_ways[i - 1] as i64;
        if f[i] == expected {
            continue;
        } else if f[i] + 1 == expected {
            coins.push(i as i32);
            for x in i..=n {
                f[x] += f[x - i];
            }
        } else {
            return vec![];
        }
    }
    coins
}

fn main() {
    println!("{:?}", find_coins(vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5]));
}

#[cfg(test)]
mod tests {
    use super::find_coins;

    #[test]
    fn example1() {
        assert_eq!(find_coins(vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5]), vec![2, 4, 6]);
    }

    #[test]
    fn example2() {
        assert_eq!(find_coins(vec![1, 2, 2, 3, 4]), vec![1, 2, 5]);
    }

    #[test]
    fn example3() {
        assert_eq!(find_coins(vec![1, 2, 3, 4, 15]), Vec::<i32>::new());
    }
}
