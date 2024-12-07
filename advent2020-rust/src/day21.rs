use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(lines: Vec<String>) {
    // contains (ingredients, allergens) tuples
    let mut ingredients_lists: Vec<(Vec<String>, Vec<String>)> = vec![];

    // all ingredients
    let mut ingredients: HashSet<String> = HashSet::new();

    // allergen => HashSet<ingredient>
    let mut maybe_allergens: HashMap<String, HashSet<String>> = HashMap::new();

    let line_re = Regex::new(r"(?<ingredients>.+) \(contains (?<allergens>.+)\)").unwrap();

    for line in lines {
        match line_re.captures(&line) {
            None => panic!("bad line"),
            Some(caps) => {
                let ingredients: HashSet<String> = caps["ingredients"]
                    .split_ascii_whitespace()
                    .map(|s| {
                        ingredients.insert(s.to_string());
                        s.to_string()
                    })
                    .collect();
                let allergens: Vec<String> = caps["allergens"]
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();

                for allergen in &allergens {
                    let maybes = maybe_allergens.get(allergen).unwrap_or(&ingredients);
                    let new_maybes: HashSet<String> =
                        maybes.intersection(&ingredients).cloned().collect();
                    maybe_allergens.insert(allergen.clone(), new_maybes);
                }

                ingredients_lists.push((ingredients.into_iter().collect(), allergens));
            }
        }
    }

    let potentially_allergenic_ingredients: HashSet<String> =
        maybe_allergens.values().flatten().cloned().collect();

    let mut non_allergenic_ingredients: HashSet<String> = ingredients.clone();
    for ingr in &potentially_allergenic_ingredients {
        non_allergenic_ingredients.remove(ingr);
    }

    let mut sum: usize = 0;
    for ingr in &non_allergenic_ingredients {
        for (ingrs, _) in &ingredients_lists {
            if ingrs.contains(ingr) {
                sum += 1;
            }
        }
    }
    println!("{sum}");

    let allergens: Vec<String> = maybe_allergens.keys().cloned().collect();
    loop {
        let mut there_were_nonuniques = false;
        for a in &allergens {
            let ingrs = maybe_allergens.get(a).unwrap().clone();
            if ingrs.len() == 1 {
                let ingr: &String = ingrs.iter().collect::<Vec<&String>>()[0];
                for (all, ings) in maybe_allergens.iter_mut() {
                    if all != a {
                        ings.remove(ingr);
                    }
                }
            } else {
                there_were_nonuniques = true;
            }
        }

        if there_were_nonuniques == false {
            break;
        }
    }

    let mut alpha_keys: Vec<String> = maybe_allergens.keys().cloned().collect::<Vec<String>>();
    alpha_keys.sort();

    let output: String = alpha_keys
        .iter()
        .flat_map(|k| maybe_allergens.get(k).unwrap())
        .cloned()
        .collect::<Vec<String>>()
        .join(",");

    println!("{output}");
}
