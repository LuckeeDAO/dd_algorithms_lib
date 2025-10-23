extern crate alloc;
#[cfg(feature = "log_tests")]
extern crate std;
use super::*;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[cfg(feature = "log_tests")]
macro_rules! test_log {
    ($($arg:tt)*) => {{
        // Only available when built with `--features log_tests`
        std::println!("{}", alloc::format!($($arg)*));
    }};
}

#[cfg(not(feature = "log_tests"))]
macro_rules! test_log {
    ($($arg:tt)*) => { { let _ = (&$($arg)*); } };
}

#[test]
fn test_calculate_fair_division_equal_weights_basic() {
    let input = [10i128, 20, 30];
    let mut output = [0i128; 3];
    let res = calculate_fair_division_equal_weights(&input, &mut output);
    assert!(res.is_ok());
    assert_eq!(output.len(), 3);
    assert_eq!(output[0] + output[1] + output[2], 0);
    test_log!("equal_weights(basic) input={:?} output={:?}", input, output);
}

#[test]
fn test_calculate_fair_division_equal_weights_with_map() {
    // key = participant id, value = bid
    let mut bids = BTreeMap::new();
    bids.insert(2usize, 30i128);
    bids.insert(0usize, 10i128);
    bids.insert(1usize, 20i128);

    // Deterministic order by key
    let values: Vec<i128> = bids.into_iter().map(|(_k, v)| v).collect();
    let mut output: Vec<i128> = Vec::with_capacity(values.len());
    output.resize(values.len(), 0i128);

    let res = calculate_fair_division_equal_weights(&values, &mut output);
    assert!(res.is_ok());
    assert_eq!(output.iter().sum::<i128>(), 0);
    test_log!("equal_weights input={:?} output={:?}", values, output);
}

#[test]
fn test_calculate_fair_division_weighted_basic() {
    let input = [10i128, 20, 30];
    let weights = [1i128, 2, 3];
    let mut output = [0i128; 3];
    let res = calculate_fair_division_weighted(&input, &weights, &mut output);
    assert!(res.is_ok());
    assert_eq!(output.len(), 3);
    assert_eq!(output[0] + output[1] + output[2], 0);
    test_log!(
        "weighted(basic) input={:?} weights={:?} output={:?}",
        input,
        weights,
        output
    );
}

#[test]
fn test_calculate_fair_division_weighted_with_map() {
    // Same keys across maps
    let mut bids = BTreeMap::new();
    bids.insert(10usize, 10i128);
    bids.insert(20usize, 20i128);
    bids.insert(30usize, 30i128);

    let mut weights = BTreeMap::new();
    weights.insert(10usize, 1i128);
    weights.insert(20usize, 2i128);
    weights.insert(30usize, 3i128);

    // Collect in the same key order
    let values_vec: Vec<i128> = bids.values().cloned().collect();
    let weights_vec: Vec<i128> = weights.values().cloned().collect();
    let mut output: Vec<i128> = Vec::with_capacity(values_vec.len());
    output.resize(values_vec.len(), 0i128);

    let res = calculate_fair_division_weighted(&values_vec, &weights_vec, &mut output);
    assert!(res.is_ok());
    assert_eq!(output.iter().sum::<i128>(), 0);
    test_log!(
        "weighted input={:?} weights={:?} output={:?}",
        values_vec,
        weights_vec,
        output
    );
}

#[test]
fn test_get_one_dd_rand_num_basic() {
    let values = [100u128, 200, 300, 400]; // 使用4个值，4是2的2次幂
    let n = values.len();
    let mut result = 0u128;
    let res = get_one_dd_rand_num(&values, n, &mut result);
    assert!(res.is_ok());
    // 异或运算的结果不需要小于n，因为异或运算的结果范围是u128
    test_log!("one_dd values={:?} n={} result={}", values, n, result);
}

#[test]
fn test_get_one_dd_rand_num_with_map() {
    let mut mp = BTreeMap::new();
    mp.insert(0usize, 100u128);
    mp.insert(2usize, 300u128);
    mp.insert(1usize, 200u128);
    mp.insert(3usize, 400u128);

    let values: Vec<u128> = mp.values().cloned().collect();
    let n = values.len();
    let mut result = 0u128;
    let res = get_one_dd_rand_num(&values, n, &mut result);
    assert!(res.is_ok());
    // 异或运算的结果不需要小于n，因为异或运算的结果范围是u128
    test_log!("one_dd(map) values={:?} n={} result={}", values, n, result);
}

#[test]
fn test_get_k_dd_rand_num_basic() {
    let group1 = [100u128, 200, 300];
    let group2 = [150u128, 250, 350];
    let group3 = [120u128, 220, 320];
    let group4 = [130u128, 230, 330]; // 添加第4组，使n=4（2的2次幂）
    let groups = [group1.as_slice(), group2.as_slice(), group3.as_slice(), group4.as_slice()];
    let mut output = [0usize; 3];
    let res = get_k_dd_rand_num(&groups, 4, 3, &mut output);
    assert!(res.is_ok());
    assert_eq!(output.len(), 3);
    // ensure uniqueness
    assert!(output[0] != output[1] && output[1] != output[2] && output[0] != output[2]);
    test_log!("k_dd(basic) groups={:?} output={:?}", groups, output);
}

#[test]
fn test_get_k_dd_rand_num_with_maps() {
    // Build three groups using maps keyed by position 0..k-1
    let mut g1 = BTreeMap::new();
    g1.insert(0usize, 100u128);
    g1.insert(1usize, 200u128);
    g1.insert(2usize, 300u128);

    let mut g2 = BTreeMap::new();
    g2.insert(0usize, 150u128);
    g2.insert(1usize, 250u128);
    g2.insert(2usize, 350u128);

    let mut g3 = BTreeMap::new();
    g3.insert(0usize, 120u128);
    g3.insert(1usize, 220u128);
    g3.insert(2usize, 320u128);

    let mut g4 = BTreeMap::new(); // 添加第4组，使n=4（2的2次幂）
    g4.insert(0usize, 130u128);
    g4.insert(1usize, 230u128);
    g4.insert(2usize, 330u128);

    // Collect each group's values in deterministic key order 0,1,2
    let group1: Vec<u128> = g1.values().cloned().collect();
    let group2: Vec<u128> = g2.values().cloned().collect();
    let group3: Vec<u128> = g3.values().cloned().collect();
    let group4: Vec<u128> = g4.values().cloned().collect();

    let groups: Vec<&[u128]> = [group1.as_slice(), group2.as_slice(), group3.as_slice(), group4.as_slice()].to_vec();
    let n = groups.len();
    let k = group1.len();
    let mut output: Vec<usize> = Vec::with_capacity(k);
    output.resize(k, 0usize);

    let res = get_k_dd_rand_num(&groups, n, k, &mut output);
    assert!(res.is_ok());
    // ensure uniqueness when n == 4, k == 3
    assert!(output[0] != output[1] && output[1] != output[2] && output[0] != output[2]);
    test_log!("k_dd groups_len={} k={} output={:?}", n, k, output);
}

#[test]
fn test_calculate_fair_division_equal_weights_invalid_len_mismatch() {
    let values = [10i128, 20, 30];
    let mut output = [0i128; 2]; // wrong len
    let res = calculate_fair_division_equal_weights(&values, &mut output);
    assert!(matches!(res, Err(Error::InvalidInput)));
    test_log!(
        "equal_weights(len_mismatch) values={:?} output_len={} -> {:?}",
        values,
        output.len(),
        res
    );
}

#[test]
fn test_calculate_fair_division_equal_weights_not_enough_participants() {
    let values = [10i128];
    let mut output = [0i128; 1];
    let res = calculate_fair_division_equal_weights(&values, &mut output);
    assert!(matches!(res, Err(Error::NotEnoughParticipants)));
    test_log!("equal_weights(not_enough) values={:?} -> {:?}", values, res);
}

#[test]
fn test_calculate_fair_division_weighted_invalid_weight_non_positive() {
    let values = [10i128, 20, 30];
    let weights = [1i128, 0, 3]; // zero weight invalid
    let mut output = [0i128; 3];
    let res = calculate_fair_division_weighted(&values, &weights, &mut output);
    assert!(matches!(res, Err(Error::InvalidInput)));
    test_log!(
        "weighted(invalid_weight) values={:?} weights={:?} -> {:?}",
        values,
        weights,
        res
    );
}

#[test]
fn test_get_one_dd_rand_num_invalid_n_mismatch() {
    let values = [1u128, 2, 3];
    let n = 2; // mismatch
    let mut out = 0u128;
    let res = get_one_dd_rand_num(&values, n, &mut out);
    assert!(matches!(res, Err(Error::InvalidInput)));
    test_log!(
        "one_dd(n_mismatch) values={:?} n={} -> {:?}",
        values,
        n,
        res
    );
}

#[test]
fn test_get_k_dd_rand_num_invalid_k_gt_n() {
    let group1 = [1u128, 2];
    let group2 = [3u128, 4];
    let groups = [group1.as_slice(), group2.as_slice()];
    let mut output = [0usize; 3]; // k=3 > n=2
    let res = get_k_dd_rand_num(&groups, 2, 3, &mut output);
    assert!(matches!(res, Err(Error::InvalidInput)));
    test_log!("k_dd(k>n) n=2 k=3 -> {:?}", res);
}

#[test]
fn test_get_k_dd_rand_num_large_n_branch() {
    // Create n=1024 groups (2的10次幂，触发大n分支), k=2
    let n = 1024usize;
    let k = 2usize;
    // Own each group's data in a Vec<Vec<u128>> so their slices live long enough
    let mut owned_groups: Vec<Vec<u128>> = Vec::with_capacity(n);
    let mut i = 0usize;
    while i < n {
        // simple deterministic values per position
        let mut g: Vec<u128> = Vec::with_capacity(k);
        g.resize(k, 0);
        g[0] = (i as u128) % 997;
        g[1] = ((i as u128) * 7) % 997;
        owned_groups.push(g);
        i += 1;
    }
    // Build slice view
    let mut groups: Vec<&[u128]> = Vec::with_capacity(n);
    let mut j = 0usize;
    while j < n {
        groups.push(owned_groups[j].as_slice());
        j += 1;
    }
    let mut output: Vec<usize> = Vec::with_capacity(k);
    output.resize(k, 0usize);
    let res = get_k_dd_rand_num(&groups, n, k, &mut output);
    assert!(res.is_ok());
    assert!(output[0] < n && output[1] < n);
    if k > 1 {
        assert!(output[0] != output[1]);
    }
    test_log!("k_dd(large_n) n={} k={} output={:?}", n, k, output);
}
