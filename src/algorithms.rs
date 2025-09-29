//! Mathematical algorithms for fair division and random number generation.

use crate::{Error, Result};

/// Calculates a super fair division for participants with equal weights.
///
/// This version works with fixed-size arrays and doesn't require heap allocation.
/// It's suitable for CosmWasm smart contracts and other no_std environments.
///
/// # Arguments
///
/// * `values` - A slice of i128 values representing the input data (bids) for each participant
/// * `output` - A mutable slice to store the calculated allocation
///
/// # Returns
///
/// * `Result<()>` - Ok if successful, Error if calculation failed
///
/// # Examples
///
/// ```
/// use dd_algorithms_lib::calculate_fair_division_equal_weights;
///
/// let input = [10, 20, 30];
/// let mut output = [0i128; 3];
/// calculate_fair_division_equal_weights(&input, &mut output).unwrap();
/// // output: [10, 20, -30] (approximately)
/// ```
pub fn calculate_fair_division_equal_weights(values: &[i128], output: &mut [i128]) -> Result<()> {
    if values.is_empty() || output.len() != values.len() {
        return Err(Error::InvalidInput);
    }

    // Check minimum participants
    if values.len() < 2 {
        return Err(Error::NotEnoughParticipants);
    }

    // Implement super fair division algorithm - equal weights version
    let n = values.len() as i128;
    let sum_v: i128 = values.iter().sum();

    // Find highest bid and its index
    let mut max_v = values[0];
    let mut max_index = 0;
    for (i, &v) in values.iter().enumerate() {
        if v > max_v {
            max_v = v;
            max_index = i;
        }
    }

    // Calculate super fair value delta = (n*maxV-sumV) / (n*n)
    let delta = match (n.checked_mul(max_v), n.checked_mul(n)) {
        (Some(n_max_v), Some(n_squared)) => match n_max_v.checked_sub(sum_v) {
            Some(numerator) => numerator / n_squared,
            None => return Err(Error::CalculationFailed),
        },
        _ => return Err(Error::CalculationFailed),
    };

    // Calculate allocation for each participant
    let mut sum_others = 0;

    for (i, &v) in values.iter().enumerate() {
        if i != max_index {
            // For non-highest bidders, allocation is v/n + delta
            let share = v / n + delta;
            output[i] = share;
            sum_others += share;
        } else {
            // Reserve space for highest bidder, fill later
            output[i] = 0;
        }
    }

    // Set highest bidder's allocation as negative sum of others
    output[max_index] = -sum_others;

    Ok(())
}

/// Calculates a super fair division for participants with different weights.
///
/// This version works with fixed-size arrays and doesn't require heap allocation.
/// It's suitable for CosmWasm smart contracts and other no_std environments.
///
/// # Arguments
///
/// * `values` - A slice of i128 values representing the input data for each participant
/// * `weights` - A slice of i128 values representing the weight of each participant
/// * `output` - A mutable slice to store the calculated allocation
///
/// # Returns
///
/// * `Result<()>` - Ok if successful, Error if calculation failed
///
/// # Examples
///
/// ```
/// use dd_algorithms_lib::calculate_fair_division_weighted;
///
/// let input = [10, 20, 30];
/// let weights = [1, 2, 3];
/// let mut output = [0i128; 3];
/// calculate_fair_division_weighted(&input, &weights, &mut output).unwrap();
/// // output: [10, 40, -50] (approximately)
/// ```
pub fn calculate_fair_division_weighted(
    values: &[i128],
    weights: &[i128],
    output: &mut [i128],
) -> Result<()> {
    if values.is_empty()
        || weights.is_empty()
        || values.len() != weights.len()
        || output.len() != values.len()
    {
        return Err(Error::InvalidInput);
    }

    // Check minimum participants
    if values.len() < 2 {
        return Err(Error::NotEnoughParticipants);
    }

    // Check that all weights are positive
    for &weight in weights {
        if weight <= 0 {
            return Err(Error::InvalidInput);
        }
    }

    // Implement super fair division algorithm - weighted version
    let total_weight: i128 = weights.iter().sum();
    let n = total_weight; // Total participants is sum of weights

    // Calculate weighted total value
    let sum_v: i128 = values
        .iter()
        .zip(weights.iter())
        .map(|(&v, &w)| v * w)
        .sum();

    // Find highest bid and its index
    let mut max_v = values[0];
    let mut max_index = 0;
    for (i, &v) in values.iter().enumerate() {
        if v > max_v {
            max_v = v;
            max_index = i;
        }
    }

    // Calculate super fair value delta = (n*maxV-sumV)/(n*n)
    let delta = match (n.checked_mul(max_v), n.checked_mul(n)) {
        (Some(n_max_v), Some(n_squared)) => match n_max_v.checked_sub(sum_v) {
            Some(numerator) => numerator / n_squared,
            None => return Err(Error::CalculationFailed),
        },
        _ => return Err(Error::CalculationFailed),
    };

    // Calculate allocation for each participant
    let mut sum_others = 0;

    for (i, (&v, &weight)) in values.iter().zip(weights.iter()).enumerate() {
        if i != max_index {
            // For non-highest bidders, allocation is (v/n+delta)*weight
            let share = (v / n + delta) * weight;
            output[i] = share;
            sum_others += share;
        } else {
            // Reserve space for highest bidder, fill later
            output[i] = 0;
        }
    }

    // Set highest bidder's allocation as negative sum of others
    output[max_index] = -sum_others;

    Ok(())
}

/// Generates a decentralized decision random number by summing all input values and taking modulo n.
///
/// This function implements a decentralized random number generation algorithm where
/// multiple participants contribute values, and the result is the sum of all values modulo n.
///
/// # Algorithm
///
/// 1. Sum all values from the input array
/// 2. During accumulation, take modulo n at each step to prevent overflow
/// 3. Return the result as u128
///
/// # Arguments
///
/// * `values` - A slice of u128 values representing input data from participants
/// * `n` - Number of values in the input array (should match values.len())
/// * `out` - A mutable reference to store the output result
///
/// # Returns
///
/// * `Result<()>` - Ok(()) if successful, Error if calculation failed
///
/// # Examples
///
/// ```
/// use dd_algorithms_lib::get_one_dd_rand_num;
///
/// let values = [100u128, 200, 300, 400, 500];
/// let n = values.len();
/// let mut result = 0u128;
/// get_one_dd_rand_num(&values, n, &mut result).unwrap();
/// // result = (100 + 200 + 300 + 400 + 500) % n
/// ```
pub fn get_one_dd_rand_num(values: &[u128], n: usize, out: &mut u128) -> Result<()> {
    // Validate input parameters
    if values.is_empty() || n == 0 {
        return Err(Error::InvalidInput);
    }

    // Verify that values.len() matches n
    if values.len() != n {
        return Err(Error::InvalidInput);
    }

    // Sum all values with optimized modulo n during accumulation
    let mut sum = 0u128;
    let n_u128 = n as u128;
    for i in 0..n {
        // Add the value
        sum += values[i];
        // Only perform modulo when sum > n to avoid unnecessary operations
        if sum >= n_u128 {
            sum = sum % n_u128;
        }
    }

    // Store the result in the output parameter
    *out = sum;
    Ok(())
}

/// Generate k decentralized random numbers from n groups of k values each
///
/// This function takes n groups of k values and generates k unique random numbers
/// using a decentralized approach where each group contributes to the final result.
/// The result is returned as an array of selected participant indices.
///
/// # Algorithm
///
/// 1. Calculate base values: output\[i\] = sum of all group\[i\] % n
/// 2. Initialize temp bool array of size n (all false)
/// 3. For each output value:
///    - Start with base_value as offset
///    - If temp\[offset\] is false, use it and mark as true
///    - If temp\[offset\] is true, increment offset and check again
///    - Wrap around if offset exceeds array size
///
/// # Arguments
///
/// * `groups` - A slice of n groups, each containing k values
/// * `n` - Number of groups (should match groups.len(), must be <= 100,000)
/// * `k` - Number of values per group (should match output.len(), must be <= 1,000)
/// * `output` - A mutable slice to store the k selected participant indices
///
/// # Constraints
///
/// * `n` must be <= 100,000 (maximum number of participants)
/// * `k` must be <= 1,000 (maximum number of selections)
/// * `k` must be <= `n` (cannot select more participants than available)
///
/// # Returns
///
/// * `Result<()>` - Ok if successful, Error if calculation failed
///
/// # Examples
///
/// ```
/// use dd_algorithms_lib::get_k_dd_rand_num;
///
/// let group1 = [100u128, 200, 300];
/// let group2 = [150u128, 250, 350];
/// let group3 = [120u128, 220, 320];
/// let groups = [group1.as_slice(), group2.as_slice(), group3.as_slice()];
/// let mut output = [0usize; 3];
/// get_k_dd_rand_num(&groups, 3, 3, &mut output).unwrap();
/// // output: [1, 2, 0] - selected participant indices
/// ```
pub fn get_k_dd_rand_num(
    groups: &[&[u128]],
    n: usize,
    k: usize,
    output: &mut [usize],
) -> Result<()> {
    // Validate input parameters
    if groups.is_empty() || n == 0 || k == 0 {
        return Err(Error::InvalidInput);
    }

    // Check constraints: n must be <= 100,000, k must be <= 1,000
    if n > 100_000 || k > 1_000 {
        return Err(Error::InvalidInput);
    }

    // Check that k <= n (cannot select more participants than available)
    if k > n {
        return Err(Error::InvalidInput);
    }

    // Verify that groups.len() matches n
    if groups.len() != n {
        return Err(Error::InvalidInput);
    }

    // Verify that output.len() matches k
    if output.len() != k {
        return Err(Error::InvalidInput);
    }

    // Check that all groups have the same length k
    for group in groups {
        if group.len() != k {
            return Err(Error::InvalidInput);
        }
    }

    // Process the first value separately
    let n_u128 = n as u128;

    // Calculate first base value
    let mut sum = 0u128;
    for group in groups {
        sum += group[0];
        // Only perform modulo when sum >= n to avoid unnecessary operations
        if sum >= n_u128 {
            sum = sum % n_u128;
        }
    }

    // First value uses the base value directly
    let first_offset = sum as usize;
    output[0] = first_offset;

    // Create a simple array to track used participants (for n <= 1000)
    let mut used = [false; 1000];
    if n <= 1000 {
        used[first_offset] = true;

        // Process subsequent values based on previous results
        for i in 1..k {
            // Calculate base value for this position
            let mut sum = 0u128;
            for group in groups {
                sum += group[i];
                // Only perform modulo when sum >= n to avoid unnecessary operations
                if sum >= n_u128 {
                    sum = sum % n_u128;
                }
            }

            // Start with base value as offset
            let mut off_t = sum as usize;

            // Find the first unused offset using direct array access
            while used[off_t] {
                off_t = (off_t + 1) % n;
            }

            // Mark this offset as used
            used[off_t] = true;

            // Set the final output value as participant index
            output[i] = off_t;
        }
    } else {
        // For large n, use the original method by checking existing output entries
        for i in 1..k {
            // Calculate base value for this position
            let mut sum = 0u128;
            for group in groups {
                sum += group[i];
                // Only perform modulo when sum >= n to avoid unnecessary operations
                if sum >= n_u128 {
                    sum = sum % n_u128;
                }
            }

            // Start with base value as offset
            let mut off_t = sum as usize;

            // Find the first unused offset by checking existing output entries
            loop {
                let mut found = false;
                for j in 0..i {
                    if output[j] == off_t {
                        found = true;
                        break;
                    }
                }

                if !found {
                    break; // Found unused offset
                }

                // Try next offset, wrapping around if necessary
                off_t = (off_t + 1) % n;
            }

            // Set the final output value as participant index
            output[i] = off_t;
        }
    }

    Ok(())
}
