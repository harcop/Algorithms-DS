/// LeetCode #3200 - Maximum Height of a Triangle
fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
    let mut ans = 0;
    for k in 0..2 {
        let mut c = [red, blue];
        let mut i = 1;
        let mut j = k;
        while i <= c[j as usize] {
            c[j as usize] -= i;
            j ^= 1;
            ans = ans.max(i);
            i += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", max_height_of_triangle(2, 4));
}

#[cfg(test)]
mod tests {
    use super::max_height_of_triangle;

    #[test]
    fn example1() {
        assert_eq!(max_height_of_triangle(2, 4), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_height_of_triangle(2, 1), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(max_height_of_triangle(1, 1), 1);
    }

    #[test]
    fn example4() {
        assert_eq!(max_height_of_triangle(10, 1), 2);
    }
}
