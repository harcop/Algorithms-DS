/// LeetCode #2782 - Number of Unique Categories
struct CategoryHandler {
    categories: Vec<i32>,
}

impl CategoryHandler {
    fn new(categories: Vec<i32>) -> Self {
        CategoryHandler { categories }
    }

    fn have_same_category(&self, a: usize, b: usize) -> bool {
        if a >= self.categories.len() || b >= self.categories.len() {
            return false;
        }
        self.categories[a] == self.categories[b]
    }
}

fn number_of_categories(n: usize, handler: &CategoryHandler) -> i32 {
    let mut p: Vec<usize> = (0..n).collect();
    fn find(p: &mut [usize], x: usize) -> usize {
        if p[x] != x {
            let root = find(p, p[x]);
            p[x] = root;
        }
        p[x]
    }
    for a in 0..n {
        for b in a + 1..n {
            if handler.have_same_category(a, b) {
                let ra = find(&mut p, a);
                let rb = find(&mut p, b);
                p[ra] = rb;
            }
        }
    }
    p.iter().enumerate().filter(|(i, &x)| *i == x).count() as i32
}

fn main() {
    let handler = CategoryHandler::new(vec![1, 1, 2, 2, 3, 3]);
    println!("{}", number_of_categories(6, &handler));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let handler = CategoryHandler::new(vec![1, 1, 2, 2, 3, 3]);
        assert_eq!(number_of_categories(6, &handler), 3);
    }

    #[test]
    fn example_two() {
        let handler = CategoryHandler::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(number_of_categories(5, &handler), 5);
    }

    #[test]
    fn example_three() {
        let handler = CategoryHandler::new(vec![1, 1, 1]);
        assert_eq!(number_of_categories(3, &handler), 1);
    }
}
