/// LeetCode #2353 - Design a Food Rating System
use std::collections::{BTreeSet, HashMap};

struct FoodRatings {
    d: HashMap<String, BTreeSet<(i32, String)>>,
    g: HashMap<String, (i32, String)>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut d: HashMap<String, BTreeSet<(i32, String)>> = HashMap::new();
        let mut g = HashMap::new();
        for i in 0..foods.len() {
            let food = foods[i].clone();
            let cuisine = cuisines[i].clone();
            let rating = ratings[i];
            d.entry(cuisine.clone())
                .or_default()
                .insert((-rating, food.clone()));
            g.insert(food, (rating, cuisine));
        }
        FoodRatings { d, g }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let (old_rating, cuisine) = self.g[&food].clone();
        self.g.insert(food.clone(), (new_rating, cuisine.clone()));
        if let Some(set) = self.d.get_mut(&cuisine) {
            set.remove(&(-old_rating, food.clone()));
            set.insert((-new_rating, food));
        }
    }

    fn highest_rated(&self, cuisine: String) -> String {
        self.d[&cuisine].iter().next().unwrap().1.clone()
    }
}

fn main() {
    let fr = FoodRatings::new(
        vec![
            "kimchi".into(),
            "miso".into(),
            "sushi".into(),
            "moussaka".into(),
            "ramen".into(),
            "bulgogi".into(),
        ],
        vec![
            "korean".into(),
            "japanese".into(),
            "japanese".into(),
            "greek".into(),
            "japanese".into(),
            "korean".into(),
        ],
        vec![9, 12, 8, 15, 14, 7],
    );
    println!("{}", fr.highest_rated("korean".into()));
}

#[cfg(test)]
mod tests {
    use super::FoodRatings;

    #[test]
    fn example_sequence() {
        let mut fr = FoodRatings::new(
            vec![
                "kimchi".into(),
                "miso".into(),
                "sushi".into(),
                "moussaka".into(),
                "ramen".into(),
                "bulgogi".into(),
            ],
            vec![
                "korean".into(),
                "japanese".into(),
                "japanese".into(),
                "greek".into(),
                "japanese".into(),
                "korean".into(),
            ],
            vec![9, 12, 8, 15, 14, 7],
        );
        assert_eq!(fr.highest_rated("korean".into()), "kimchi");
        assert_eq!(fr.highest_rated("japanese".into()), "ramen");
        fr.change_rating("sushi".into(), 16);
        assert_eq!(fr.highest_rated("japanese".into()), "sushi");
        fr.change_rating("ramen".into(), 16);
        assert_eq!(fr.highest_rated("japanese".into()), "ramen");
    }
}
