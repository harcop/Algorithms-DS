/// LeetCode #1912 - Design Movie Rental System
use std::collections::{BTreeSet, HashMap};

pub struct MovieRentingSystem {
    available: HashMap<i32, BTreeSet<(i32, i32)>>,
    price_map: HashMap<i64, i32>,
    rented: BTreeSet<(i32, i32, i32)>,
}

impl MovieRentingSystem {
    fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut available: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::new();
        let mut price_map = HashMap::new();
        for e in entries {
            let shop = e[0];
            let movie = e[1];
            let price = e[2];
            available
                .entry(movie)
                .or_default()
                .insert((price, shop));
            price_map.insert(Self::key(shop, movie), price);
        }
        MovieRentingSystem {
            available,
            price_map,
            rented: BTreeSet::new(),
        }
    }

    fn key(shop: i32, movie: i32) -> i64 {
        (shop as i64) << 30 | movie as i64
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.available
            .get(&movie)
            .into_iter()
            .flat_map(|set| set.iter().take(5).map(|&(_, shop)| shop))
            .collect()
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = self.price_map[&Self::key(shop, movie)];
        self.available
            .get_mut(&movie)
            .unwrap()
            .remove(&(price, shop));
        self.rented.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = self.price_map[&Self::key(shop, movie)];
        self.rented.remove(&(price, shop, movie));
        self.available
            .entry(movie)
            .or_default()
            .insert((price, shop));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented
            .iter()
            .take(5)
            .map(|&(_, shop, movie)| vec![shop, movie])
            .collect()
    }
}

fn main() {
    let obj = MovieRentingSystem::new(
        3,
        vec![
            vec![0, 1, 5],
            vec![0, 2, 6],
            vec![0, 3, 7],
            vec![1, 1, 4],
            vec![1, 2, 7],
            vec![2, 1, 5],
        ],
    );
    println!("{:?}", obj.search(1));
}

#[cfg(test)]
mod tests {
    use super::MovieRentingSystem;

    #[test]
    fn example_one() {
        let mut obj = MovieRentingSystem::new(
            3,
            vec![
                vec![0, 1, 5],
                vec![0, 2, 6],
                vec![0, 3, 7],
                vec![1, 1, 4],
                vec![1, 2, 7],
                vec![2, 1, 5],
            ],
        );
        assert_eq!(obj.search(1), vec![1, 0, 2]);
        obj.rent(0, 1);
        obj.rent(1, 2);
        assert_eq!(obj.report(), vec![vec![0, 1], vec![1, 2]]);
        obj.drop(1, 2);
        assert_eq!(obj.search(2), vec![0, 1]);
    }
}
