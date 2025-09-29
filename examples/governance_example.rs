use dd_algorithms_lib::{
    // Fair Division
    calculate_fair_division_equal_weights,
    calculate_fair_division_weighted,

    get_k_dd_rand_num,
    // Random Generation
    get_one_dd_rand_num,
};

fn main() {
    println!("=== Decentralized Decision Library Example ===\n");

    // 1. Fair Division Example
    println!("1. Fair Division Example:");

    // Equal weights
    let bids = [100i128, 200, 300, 400];
    let mut allocation = [0i128; 4];
    calculate_fair_division_equal_weights(&bids, &mut allocation).unwrap();
    println!("   Equal weights allocation: {:?}", allocation);
    println!(
        "   Sum check: {} (should be 0)",
        allocation.iter().sum::<i128>()
    );

    // Weighted division
    let weights = [1i128, 2, 3, 4];
    calculate_fair_division_weighted(&bids, &weights, &mut allocation).unwrap();
    println!("   Weighted allocation: {:?}", allocation);
    println!(
        "   Sum check: {} (should be 0)",
        allocation.iter().sum::<i128>()
    );

    // 2. Random Selection Example
    println!("\n2. Random Selection Example:");

    // Single random number
    let values = [100u128, 200, 300, 400, 500];
    let mut result = 0u128;
    get_one_dd_rand_num(&values, values.len(), &mut result).unwrap();
    println!("   Single random number: {}", result);

    // Multiple random numbers
    let group1 = [100u128, 200, 300];
    let group2 = [150u128, 250, 350];
    let group3 = [120u128, 220, 320];
    let groups = [group1.as_slice(), group2.as_slice(), group3.as_slice()];
    let mut selected = [0usize; 3];
    get_k_dd_rand_num(&groups, 3, 3, &mut selected).unwrap();
    println!("   Selected participants: {:?}", selected);

    println!("\n=== Example Complete ===");
}
