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
    return get_min_ore_needed_one_fuel(filename);
}

fn get_min_ore_needed_one_fuel(filename: String) -> u64 {
    let reactions = get_reactions_from_filename(filename);
    // Get fuel reaction and do initial checks
    let fuel_reaction = reactions.get("FUEL").unwrap().clone();
    let (min_ore_needed, _) =
        get_ore_needed_for_reaction(&reactions.clone(), fuel_reaction, &mut HashMap::new());
    return min_ore_needed;
}

/// Calculates solution for Day 14 Part 2 challenge.
pub fn solution_part_2(filename: String) -> u64 {
    let mut ore_remaining = 10e12 as u64;
    let mut fuel_made = 0;
    let mut remainders = HashMap::<String, u64>::new();
    let reactions = get_reactions_from_filename(filename.clone());
    let fuel_reaction = reactions.get("FUEL").unwrap();
    let min_ore_needed = get_min_ore_needed_one_fuel(filename.clone());
    while ore_remaining > min_ore_needed {
        let (_, new_remainders) =
            get_ore_needed_for_reaction(&reactions, fuel_reaction.clone(), &mut remainders);
        remainders = new_remainders.clone();
        fuel_made += 1;
        ore_remaining -= min_ore_needed;
        if fuel_made % 10 == 0 {
            println!("Fuel made: {}", fuel_made);
        }
    }
    return fuel_made;
}

fn get_reactions_from_filename(filename: String) -> HashMap<String, ChemicalReaction> {
    let mut file = fs::open_file(filename);
    let raw_input = io::read_file_to_string(&mut file);
    let mut reactions = HashMap::<String, ChemicalReaction>::new();
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
        let rhs_split: Vec<String> = side_split[1]
            .trim()
            .split(" ")
            .map(|x| String::from(x))
            .collect();
        if rhs_split.len() != 2 {
            panic!("Bad material format on RHS.");
        }
        let quantity = rhs_split[0].parse::<u64>().unwrap();
        let output_material = Material {
            name: rhs_split[1].clone(),
            quantity,
        };
        // Create the chemical reaction
        let reaction = ChemicalReaction {
            input: lhs_materials.clone(),
            output: output_material.clone(),
        };
        // Check if there is more than one reaction that can produce the same output material
        let result = reactions.insert(rhs_split[1].clone(), reaction);
        match result {
            None => (),
            _ => panic!(
                "More than one formula to produce chemical: {}",
                output_material.name
            ),
        }
    }
    return reactions;
}

/// Calculates how much ORE is needed to produce the output of the given target reaction.
///
/// Extra amounts of materials remaining after each reaction is run are tracked between runs. This
/// is done so that this extra amount can be used if enough is held, rather than making more of the
/// material from raw ORE.
fn get_ore_needed_for_reaction(
    reactions_record: &HashMap<String, ChemicalReaction>,
    target_reaction: ChemicalReaction,
    remainders: &HashMap<String, u64>,
) -> (u64, HashMap<String, u64>) {
    if target_reaction.input.len() == 1 && target_reaction.input[0].name == "ORE" {
        return (target_reaction.input[0].quantity, remainders.clone());
    }
    let mut total_ore_needed = 0;
    let mut remainders = remainders.clone();

    for input_material in target_reaction.input {
        let input_reaction = reactions_record.get(&input_material.name).unwrap();
        if !remainders.contains_key(&input_material.name) {
            remainders.insert(input_material.name.clone(), 0);
        }
        let stored_amount = *remainders.get(&input_material.name).unwrap();
        if stored_amount > input_material.quantity {
            *remainders.get_mut(&input_material.name).unwrap() -= input_material.quantity;
        } else {
            // Work out how much of input material we need to produce based on amount already held
            let desired_qty =
                input_material.quantity - remainders.get(&input_material.name).unwrap();
            let reps = (desired_qty as f64 / input_reaction.output.quantity as f64).ceil() as u64;
            // Update remaining amount stored based on amount due to be produced
            let produced_amount = reps * input_reaction.output.quantity;
            let amount_remaining = stored_amount + produced_amount - input_material.quantity;
            *remainders.get_mut(&input_material.name).unwrap() = amount_remaining;
            let mut ore_needed = 0;
            // Run repeats so remainders can be updated on each run - in case more than enough is
            // produced from a previous run. WE NEED MINIMUM ORE!!!
            for _ in 0..reps {
                let (ore, new_remainders) = get_ore_needed_for_reaction(
                    &reactions_record,
                    input_reaction.clone(),
                    &mut remainders,
                );
                ore_needed += ore;
                remainders = new_remainders.clone();
            }
            total_ore_needed += ore_needed;
        }
    }
    return (total_ore_needed, remainders.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1_example_01() {
        let result = solution_part_1(String::from("./input/day_14/test/test_01.txt"));
        assert_eq!(31, result);
    }

    #[test]
    pub fn test_p1_example_02() {
        let result = solution_part_1(String::from("./input/day_14/test/test_02.txt"));
        assert_eq!(165, result);
    }

    #[test]
    pub fn test_p1_example_03() {
        let result = solution_part_1(String::from("./input/day_14/test/test_03.txt"));
        assert_eq!(13312, result);
    }

    #[test]
    pub fn test_p1_example_04() {
        let result = solution_part_1(String::from("./input/day_14/test/test_04.txt"));
        assert_eq!(180697, result);
    }

    #[test]
    pub fn test_p1_example_05() {
        let result = solution_part_1(String::from("./input/day_14/test/test_05.txt"));
        assert_eq!(2210736, result);
    }

    #[test]
    pub fn test_p1_solution() {
        let result = solution_part_1(String::from("./input/day_14/input.txt"));
        assert_eq!(278404, result);
    }

    #[test]
    pub fn test_p2_example_03() {
        let result = solution_part_2(String::from("./input/day_14/test/test_03.txt"));
        assert_eq!(82892753, result);
    }

    #[test]
    pub fn test_p2_example_04() {
        let result = solution_part_2(String::from("./input/day_14/test/test_04.txt"));
        assert_eq!(5586022, result);
    }

    #[test]
    pub fn test_p2_example_05() {
        let result = solution_part_2(String::from("./input/day_14/test/test_05.txt"));
        assert_eq!(460664, result);
    }
}
