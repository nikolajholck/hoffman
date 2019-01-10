extern crate hoffman;

use hoffman::*;
use std::time::Instant;

fn main() {
    let recipes: Vec<Recipe> = (1..=900).map(|i| {
        Recipe::load_json(&String::from("res"), &format!("cube-4d-{}", i))
    }).collect();

    let limit = 50;
    let start = Instant::now();
    for a in 1..limit {
        for b in a..limit {
            let sub_time = Instant::now();
            for c in b..limit {
                for d in c..limit {
                    let dimension_tuple = vec!(a, b, c, d);
                    if !validate_recipes(&recipes, &dimension_tuple) {
                        panic!("Counter-example using dimension tuple {:?}!", dimension_tuple);
                    }
                }
            }
            println!("({}, {}, _, _) dimension tuples passed in {} seconds.", a, b, sub_time.elapsed().as_secs());
        }
    }
    println!("All dimension tuples passed in {} seconds.", start.elapsed().as_secs());
}

fn validate_recipes(recipes: &Vec<Recipe>, dimension_tuple: &DimensionTuple) -> bool {
    recipes.iter().all(|recipe| {
        let recipe_builder = RecipeBuilder::generate(recipe, vec!(dimension_tuple.clone()));
        recipe_builder.validate()
    })
}
