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

    for (_, v) in reactions.into_iter() {
        println!("{:?}", v);
    }

    unimplemented!();
}
