extern crate hoffman;

use hoffman::*;

fn main() {
    let recipes: Vec<Recipe> = (0..21).map(|i| {
        Recipe::load_json(&String::from("res"), &format!("3D Packing {}", i))
    }).collect();
    let limit = 100;
    for a in 1..limit {
        for b in a..limit {
            for c in b..limit {
                let dimension_tuple = vec!(a, b, c);
                if !validate_recipes(&recipes, &dimension_tuple) {
                    panic!("Counter-example using dimension tuple {:?}!", dimension_tuple);
                }
            }
        }
        println!("({}, _, _) dimension tuples passed.", a);
    }
}

fn validate_recipes(recipes: &Vec<Recipe>, dimension_tuple: &DimensionTuple) -> bool {
    recipes.iter().all(|recipe| {
        let recipe_builder = RecipeBuilder::generate(recipe, vec!(dimension_tuple.clone()));
        recipe_builder.validate()
    })
}
