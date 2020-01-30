use primes;
use std::collections::HashMap;

/// Calculates the Lowest Common Multiple (LCM) for the given values.HashMap
///
/// The LCM for a set of positive integers is the smallest number that can be evenly divided by each
/// of the values in the set.
pub fn calculate_lcm(input: Vec<u64>) -> u128 {
    // Calculate prime factors
    let mut prime_factors: Vec<Vec<u64>> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let p_fact = primes::factors(input[i]);
        prime_factors.push(p_fact);
    }
    // Count prime factors
    let mut prime_factors_count: Vec<HashMap<u64, u32>> = Vec::with_capacity(prime_factors.len());
    for i in 0..prime_factors.len() {
        let mut count = HashMap::<u64, u32>::new();
        for prime in prime_factors[i].clone() {
            *count.entry(prime).or_insert(0) += 1;
        }
        prime_factors_count.push(count);
    }
    // Work out most times each prime observed occurs for one of the numbers
    let mut result_count = HashMap::<u64, u32>::new();
    for i in 0..prime_factors_count.len() {
        for (prime, count) in prime_factors_count[i].clone().into_iter() {
            let mut max_count = count;
            // Check if prime factor occurs more for one of the other input values
            for j in 0..prime_factors_count.len() {
                if i == j {
                    continue;
                }
                if *prime_factors_count[j].get(&prime).unwrap_or(&0) > max_count {
                    max_count = *prime_factors_count[j].get(&prime).unwrap();
                }
            }
            result_count.insert(prime, max_count);
        }
    }
    // Calculate the LCM
    let mut lcm: u128 = 1;
    for (prime, max_count) in result_count.into_iter() {
        lcm *= prime.pow(max_count) as u128;
    }
    return lcm;
}
