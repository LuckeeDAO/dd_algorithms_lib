use dd_algorithms_lib::{
    calculate_fair_division_equal_weights, calculate_fair_division_weighted, get_k_dd_rand_num,
    get_one_dd_rand_num,
};

fn main() {
    println!("=== Function Names Test ===\n");

    // Test 1: Equal weights fair division
    println!("1. Testing calculate_fair_division_equal_weights:");
    let bids = [100i128, 200, 300];
    let mut allocation = [0i128; 3];
    match calculate_fair_division_equal_weights(&bids, &mut allocation) {
        Ok(()) => {
            println!("   ✓ Function name updated successfully");
            println!("   Result: {:?}", allocation);
            println!("   Sum: {} (should be 0)", allocation.iter().sum::<i128>());
        }
        Err(e) => println!("   ✗ Error: {:?}", e),
    }

    // Test 2: Weighted fair division
    println!("\n2. Testing calculate_fair_division_weighted:");
    let weights = [1i128, 2, 3];
    match calculate_fair_division_weighted(&bids, &weights, &mut allocation) {
        Ok(()) => {
            println!("   ✓ Function name updated successfully");
            println!("   Result: {:?}", allocation);
            println!("   Sum: {} (should be 0)", allocation.iter().sum::<i128>());
        }
        Err(e) => println!("   ✗ Error: {:?}", e),
    }

    // Test 3: Single random number
    println!("\n3. Testing get_one_dd_rand_num:");
    let values = [100u128, 200, 300, 400, 500];
    let mut result = 0u128;
    match get_one_dd_rand_num(&values, values.len(), &mut result) {
        Ok(()) => {
            println!("   ✓ Function name unchanged (correct)");
            println!("   Result: {}", result);
        }
        Err(e) => println!("   ✗ Error: {:?}", e),
    }

    // Test 4: Multiple random numbers
    println!("\n4. Testing get_k_dd_rand_num:");
    let group1 = [100u128, 200, 300];
    let group2 = [150u128, 250, 350];
    let group3 = [120u128, 220, 320];
    let groups = [group1.as_slice(), group2.as_slice(), group3.as_slice()];
    let mut selected = [0usize; 3];
    match get_k_dd_rand_num(&groups, 3, 3, &mut selected) {
        Ok(()) => {
            println!("   ✓ Function name unchanged (correct)");
            println!("   Selected: {:?}", selected);
        }
        Err(e) => println!("   ✗ Error: {:?}", e),
    }

    println!("\n=== All function names updated successfully! ===");
    println!(
        "✓ calculate_fair_division_equal_weights_no_std → calculate_fair_division_equal_weights"
    );
    println!("✓ calculate_fair_division_weighted_no_std → calculate_fair_division_weighted");
    println!("\n✓ Core algorithms:");
    println!("✓ get_one_dd_rand_num (unchanged)");
    println!("✓ get_k_dd_rand_num (unchanged)");
}
