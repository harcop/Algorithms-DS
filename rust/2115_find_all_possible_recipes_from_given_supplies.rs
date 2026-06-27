/// LeetCode #2115 - Find All Possible Recipes from Given Supplies
use std::collections::{HashMap, HashSet, VecDeque};

fn find_all_recipes(
    recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
) -> Vec<String> {
    let recipe_set: HashSet<String> = recipes.iter().cloned().collect();
    let mut indegree: HashMap<String, i32> = HashMap::new();
    let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

    for (recipe, needs) in recipes.iter().zip(ingredients.iter()) {
        indegree.insert(recipe.clone(), 0);
        for ingredient in needs {
            if recipe_set.contains(ingredient) {
                *indegree.get_mut(recipe).unwrap() += 1;
                dependents
                    .entry(ingredient.clone())
                    .or_default()
                    .push(recipe.clone());
            }
        }
    }

    let mut available: HashSet<String> = supplies.into_iter().collect();
    let mut q = VecDeque::new();
    for (recipe, needs) in recipes.iter().zip(ingredients.iter()) {
        if indegree[recipe] == 0
            && needs
                .iter()
                .all(|ingredient| available.contains(ingredient))
        {
            q.push_back(recipe.clone());
        }
    }

    let mut ans = Vec::new();
    while let Some(recipe) = q.pop_front() {
        if !available.insert(recipe.clone()) {
            continue;
        }
        ans.push(recipe.clone());

        if let Some(next_recipes) = dependents.get(&recipe) {
            for next in next_recipes {
                if let Some(count) = indegree.get_mut(next) {
                    *count -= 1;
                    let idx = recipes.iter().position(|recipe| recipe == next).unwrap();
                    if *count == 0
                        && ingredients[idx]
                            .iter()
                            .all(|ingredient| available.contains(ingredient))
                    {
                        q.push_back(next.clone());
                    }
                }
            }
        }
    }

    ans
}

fn main() {
    println!(
        "{:?}",
        find_all_recipes(
            vec!["bread".into()],
            vec![vec!["yeast".into(), "flour".into()]],
            vec!["yeast".into(), "flour".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_all_recipes;

    #[test]
    fn example_one() {
        assert_eq!(
            find_all_recipes(
                vec!["bread".into()],
                vec![vec!["yeast".into(), "flour".into()]],
                vec!["yeast".into(), "flour".into()],
            ),
            vec!["bread"]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_all_recipes(
                vec!["bread".into(), "sandwich".into()],
                vec![
                    vec!["yeast".into(), "flour".into()],
                    vec!["bread".into(), "meat".into()]
                ],
                vec!["yeast".into(), "flour".into(), "meat".into()],
            ),
            vec!["bread", "sandwich"]
        );
    }

    #[test]
    fn skips_missing_ingredients() {
        assert!(find_all_recipes(
            vec!["bread".into()],
            vec![vec!["yeast".into(), "flour".into()]],
            vec!["yeast".into()],
        )
        .is_empty());
    }
}
