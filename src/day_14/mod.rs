use super::utils::fs;
use super::utils::io;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Material {
    name: String,
    quantity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ChemicalReaction {
    input: Vec<Material>,
    output: Material,
}

/// Calculates solution for Day 14 Part 1 challenge.
pub fn solution_part_1(filename: String) -> u64 {
    let mut file = fs::open_file(filename);
    let raw_input = io::read_file_to_string(&mut file);
    let mut reactions = HashMap::<String, Vec<ChemicalReaction>>::new();
    // Read lines into chemical reactions
    for line in raw_input.lines() {
        let line = line.trim();
        let side_split: Vec<String> = line.split("=>").map(|x| String::from(x)).collect();
        if side_split.len() != 2 {
            panic!("Bad number of sides for reaction: {}", side_split.len());
        }
        // Split up the materials on LHS and collect into vector
        let lhs_split: Vec<String> = side_split[0].split(",").map(|x| String::from(x)).collect();
        let mut lhs_materials: Vec<Material> = vec![];
        for item in lhs_split {
            let item = item.trim();
            let material_split: Vec<String> = item.split(' ').map(|x| String::from(x)).collect();
            if material_split.len() != 2 {
                panic!("Bad material format on LHS");
            }
            let quantity = material_split[0].parse::<u64>().unwrap();
            let material = Material {
                name: material_split[1].clone(),
                quantity,
            };
            lhs_materials.push(material);
        }
        // Get the material type and quantity from RHS
        let rhs_split: Vec<String> = side_split[1].trim().split(" ").map(|x| String::from(x)).collect();
        if rhs_split.len() != 2 {
            panic!("Bad material format on RHS.");
        }
        let quantity = rhs_split[0].parse::<u64>().unwrap();
        let output_material = Material {
            name: rhs_split[1].clone(),
            quantity,
        };
        let reaction = ChemicalReaction {
            input: lhs_materials.clone(),
            output: output_material,
        };
        // Add reaction
        if !reactions.contains_key(&rhs_split[1]) {
            reactions.insert(rhs_split[1].clone(), vec![reaction]);
        } else {
            reactions.get_mut(&rhs_split[1]).unwrap().push(reaction);
        }
    }

    // Get fuel reaction and do initial checks
    let fuel_reaction = reactions.get("FUEL").unwrap()[0].clone();
    let min_ore_needed = get_ore_needed_for_reaction(&reactions.clone(), fuel_reaction, HashMap::new());

    // for (_, v) in reactions.into_iter() {
    //     println!("{:?}", v);
    // }

    return min_ore_needed;
}

fn get_ore_needed_for_reaction(reactions_record: &HashMap::<String, Vec<ChemicalReaction>>, target_reaction: ChemicalReaction, remainders: HashMap<String, u64>) -> u64 {
    if target_reaction.input.len() == 1 && target_reaction.input[0].name == "ORE" {
        return target_reaction.input[0].quantity;
    }

    let mut total_ore_needed = 0;
    let mut remainders = remainders.clone();
    for input_material in target_reaction.input {
        // Get reactions that can produce the required material and calculate how many times needed
        let possible_reactions = reactions_record.get(&input_material.name).unwrap().clone();
        let mut min_ore_needed = u64::max_value();
        for poss in possible_reactions {
            let desired_qty = input_material.quantity - remainders.get(&input_material.name).unwrap_or(&0);
            let reps = (desired_qty + poss.output.quantity/2) / poss.output.quantity;
            if !remainders.contains_key(&poss.output.name) {
                remainders.insert(poss.output.name.clone(), reps * poss.output.quantity);
            } else {
                *remainders.get_mut(&poss.output.name).unwrap() += reps * poss.output.quantity;
            }
            // Subtract the amount used by the current target reaction
            *remainders.get_mut(&poss.output.name).unwrap() -= desired_qty;
            // Repeat the reaction and find how much ore is needed
            let ore_needed = reps * get_ore_needed_for_reaction(&reactions_record, poss.clone(), remainders.clone());
            if ore_needed < min_ore_needed {
                min_ore_needed = ore_needed;
            }
        }
        total_ore_needed += min_ore_needed;
    }

    return total_ore_needed;
}
