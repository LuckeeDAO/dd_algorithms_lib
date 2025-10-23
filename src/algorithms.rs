//! 公平分配和随机数生成的数学算法。

use crate::{Error, Result};

#[inline]
fn is_whitelisted(idx: usize, whitelist: &[usize]) -> bool {
    for &w in whitelist {
        if w == idx {
            return true;
        }
    }
    false
}

/// 为权重相等的参与者计算超级公平分配。
///
/// 此版本使用固定大小数组，不需要堆分配。
/// 适用于CosmWasm智能合约和其他no_std环境。
///
/// # 参数
///
/// * `values` - 表示每个参与者输入数据（出价）的i128值切片
/// * `output` - 用于存储计算分配结果的可变切片
///
/// # 返回值
///
/// * `Result<()>` - 成功时返回Ok，计算失败时返回Error
///
/// # 示例
///
/// ```
/// use dd_algorithms_lib::calculate_fair_division_equal_weights;
///
/// let input = [10, 20, 30];
/// let mut output = [0i128; 3];
/// calculate_fair_division_equal_weights(&input, &mut output).unwrap();
/// // output: [10, 20, -30] (大约)
/// ```
pub fn calculate_fair_division_equal_weights(values: &[i128], output: &mut [i128]) -> Result<()> {
    if values.is_empty() || output.len() != values.len() {
        return Err(Error::InvalidInput);
    }

    // 检查最少参与者数量
    if values.len() < 2 {
        return Err(Error::NotEnoughParticipants);
    }

    // 实现超级公平分配算法 - 等权重版本
    let n = values.len() as i128;
    let sum_v: i128 = values.iter().sum();

    // 找到最高出价及其索引
    let mut max_v = values[0];
    let mut max_index = 0;
    for (i, &v) in values.iter().enumerate() {
        if v > max_v {
            max_v = v;
            max_index = i;
        }
    }

    // 计算超级公平值增量 delta = (n*maxV-sumV) / (n*n)
    let delta = match (n.checked_mul(max_v), n.checked_mul(n)) {
        (Some(n_max_v), Some(n_squared)) => match n_max_v.checked_sub(sum_v) {
            Some(numerator) => numerator / n_squared,
            None => return Err(Error::CalculationFailed),
        },
        _ => return Err(Error::CalculationFailed),
    };

    // 计算每个参与者的分配
    let mut sum_others = 0;

    for (i, &v) in values.iter().enumerate() {
        if i != max_index {
            // 对于非最高出价者，分配为 v/n + delta
            let share = v / n + delta;
            output[i] = share;
            sum_others += share;
        } else {
            // 为最高出价者保留空间，稍后填充
            output[i] = 0;
        }
    }

    // 将最高出价者的分配设置为其他参与者分配的负和
    output[max_index] = -sum_others;

    Ok(())
}

/// 为权重不同的参与者计算超级公平分配。
///
/// 此版本使用固定大小数组，不需要堆分配。
/// 适用于CosmWasm智能合约和其他no_std环境。
///
/// # 参数
///
/// * `values` - 表示每个参与者输入数据的i128值切片
/// * `weights` - 表示每个参与者权重的i128值切片
/// * `output` - 用于存储计算分配结果的可变切片
///
/// # 返回值
///
/// * `Result<()>` - 成功时返回Ok，计算失败时返回Error
///
/// # 示例
///
/// ```
/// use dd_algorithms_lib::calculate_fair_division_weighted;
///
/// let input = [10, 20, 30];
/// let weights = [1, 2, 3];
/// let mut output = [0i128; 3];
/// calculate_fair_division_weighted(&input, &weights, &mut output).unwrap();
/// // output: [10, 40, -50] (大约)
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

    // 检查最少参与者数量
    if values.len() < 2 {
        return Err(Error::NotEnoughParticipants);
    }

    // 检查所有权重都为正数
    for &weight in weights {
        if weight <= 0 {
            return Err(Error::InvalidInput);
        }
    }

    // 实现超级公平分配算法 - 加权版本
    let total_weight: i128 = weights.iter().sum();
    let n = total_weight; // 总参与者数是权重之和

    // 计算加权总值
    let sum_v: i128 = values
        .iter()
        .zip(weights.iter())
        .map(|(&v, &w)| v * w)
        .sum();

    // 找到最高出价及其索引
    let mut max_v = values[0];
    let mut max_index = 0;
    for (i, &v) in values.iter().enumerate() {
        if v > max_v {
            max_v = v;
            max_index = i;
        }
    }

    // 计算超级公平值增量 delta = (n*maxV-sumV)/(n*n)
    let delta = match (n.checked_mul(max_v), n.checked_mul(n)) {
        (Some(n_max_v), Some(n_squared)) => match n_max_v.checked_sub(sum_v) {
            Some(numerator) => numerator / n_squared,
            None => return Err(Error::CalculationFailed),
        },
        _ => return Err(Error::CalculationFailed),
    };

    // 计算每个参与者的分配
    let mut sum_others = 0;

    for (i, (&v, &weight)) in values.iter().zip(weights.iter()).enumerate() {
        if i != max_index {
            // 对于非最高出价者，分配为 (v/n+delta)*weight
            let share = (v / n + delta) * weight;
            output[i] = share;
            sum_others += share;
        } else {
            // 为最高出价者保留空间，稍后填充
            output[i] = 0;
        }
    }

    // 将最高出价者的分配设置为其他参与者分配的负和
    output[max_index] = -sum_others;

    Ok(())
}

/// 通过异或运算生成去中心化决策随机数。
///
/// 此函数实现去中心化随机数生成算法，其中多个参与者贡献值，
/// 结果是所有值的异或运算结果。使用异或运算可以保证概率相等，
/// 只要保证总数n是2的n次幂即可。
///
/// # 算法
///
/// 1. 对输入数组中的所有值进行异或运算
/// 2. 异或运算具有交换律和结合律，结果与顺序无关
/// 3. 当n是2的n次幂时，异或运算能保证每个结果出现的概率相等
/// 4. 返回u128类型的结果
///
/// # 参数
///
/// * `values` - 表示参与者输入数据的u128值切片
/// * `n` - 输入数组中的值数量（应与values.len()匹配，且必须是2的n次幂）
/// * `out` - 用于存储输出结果的可变引用
///
/// # 返回值
///
/// * `Result<()>` - 成功时返回Ok(())，计算失败时返回Error
///
/// # 示例
///
/// ```
/// use dd_algorithms_lib::get_one_dd_rand_num;
///
/// let values = [100u128, 200, 300, 400]; // 使用4个值，4是2的2次幂
/// let n = values.len();
/// let mut result = 0u128;
/// get_one_dd_rand_num(&values, n, &mut result).unwrap();
/// // result = 100 ^ 200 ^ 300 ^ 400
/// ```
pub fn get_one_dd_rand_num(values: &[u128], n: usize, out: &mut u128) -> Result<()> {
    // 验证输入参数
    if values.is_empty() || n == 0 {
        return Err(Error::InvalidInput);
    }

    // 验证values.len()与n匹配
    if values.len() != n {
        return Err(Error::InvalidInput);
    }

    // 验证n是否为2的n次幂
    if n & (n - 1) != 0 {
        return Err(Error::InvalidInput);
    }

    // 使用异或运算对所有值进行计算
    let mut result = 0u128;
    for i in 0..n {
        result ^= values[i];
    }

    // 将结果存储在输出参数中
    *out = result;
    Ok(())
}

/// 通过异或运算生成去中心化决策随机数（模仿福彩3D随机数算法）。
///
/// 此函数实现去中心化随机数生成算法，用于模仿福彩3D的随机数生成机制。
/// 多个参与者贡献值，结果是所有值的异或运算结果。使用异或运算可以保证概率相等，
/// 只要保证k是2的n次幂即可。
///
/// # 算法
///
/// 1. 对输入数组中的所有值进行异或运算
/// 2. 异或运算具有交换律和结合律，结果与顺序无关
/// 3. 当k是2的n次幂时，异或运算能保证每个结果出现的概率相等
/// 4. 返回u128类型的结果
///
/// # 参数
///
/// * `values` - 表示参与者输入数据的u128值切片
/// * `n` - 输入数组中的值数量（应与values.len()匹配）
/// * `k` - 用于验证的数值（必须大于0且必须是2的n次幂，通常为福彩3D的号码范围）
/// * `out` - 用于存储输出结果的可变引用
///
/// # 返回值
///
/// * `Result<()>` - 成功时返回Ok(())，计算失败时返回Error
///
/// # 示例
///
/// ```
/// use dd_algorithms_lib::get_one_dd_3d_rand_num;
///
/// let values = [100u128, 200, 300, 400]; // 使用4个值
/// let n = values.len();
/// let k = 8; // 8是2的3次幂，福彩3D号码范围0-7
/// let mut result = 0u128;
/// get_one_dd_3d_rand_num(&values, n, k, &mut result).unwrap();
/// // result = 100 ^ 200 ^ 300 ^ 400
/// ```
pub fn get_one_dd_3d_rand_num(values: &[u128], n: usize, k: usize, out: &mut u128) -> Result<()> {
    // 验证输入参数
    if values.is_empty() || n == 0 || k == 0 {
        return Err(Error::InvalidInput);
    }

    // 验证values.len()与n匹配
    if values.len() != n {
        return Err(Error::InvalidInput);
    }

    // 验证k是否为2的n次幂
    if k & (k - 1) != 0 {
        return Err(Error::InvalidInput);
    }

    // 使用异或运算对所有值进行计算
    let mut result = 0u128;
    for i in 0..n {
        result ^= values[i];
    }

    // 将结果存储在输出参数中
    *out = result;
    Ok(())
}

/// 从n组每组k个值生成k个去中心化随机数
///
/// 此函数接受n组每组k个值，并使用去中心化方法生成k个唯一随机数，
/// 其中每组都对最终结果做出贡献。使用异或运算可以保证概率相等，
/// 只要保证n是2的n次幂即可。结果以选中的参与者索引数组形式返回。
///
/// # 算法
///
/// 1. 计算基础值：output\[i\] = 所有group\[i\]的异或运算结果
/// 2. 初始化大小为n的临时布尔数组（全部为false）
/// 3. 对于每个输出值：
///    - 以base_value作为偏移量开始
///    - 如果temp\[offset\]为false，使用它并标记为true
///    - 如果temp\[offset\]为true，增加偏移量并再次检查
///    - 如果偏移量超过数组大小则回绕
///
/// # 参数
///
/// * `groups` - n组的切片，每组包含k个值
/// * `n` - 组数（应与groups.len()匹配，必须 <= 100,000且必须是2的n次幂）
/// * `k` - 每组的值的数量（应与output.len()匹配，必须 <= 1,000）
/// * `output` - 用于存储k个选中参与者索引的可变切片
///
/// # 约束条件
///
/// * `n` 必须 <= 100,000（最大参与者数）
/// * `n` 必须是2的n次幂（保证异或运算的概率相等性）
/// * `k` 必须 <= 1,000（最大选择数）
/// * `k` 必须 <= `n`（不能选择比可用参与者更多的参与者）
///
/// # 返回值
///
/// * `Result<()>` - 成功时返回Ok，计算失败时返回Error
///
/// # 示例
///
/// ```
/// use dd_algorithms_lib::get_k_dd_rand_num;
///
/// let group1 = [100u128, 200, 300];
/// let group2 = [150u128, 250, 350];
/// let group3 = [120u128, 220, 320];
/// let group4 = [130u128, 230, 330];
/// let groups = [group1.as_slice(), group2.as_slice(), group3.as_slice(), group4.as_slice()];
/// let mut output = [0usize; 3];
/// get_k_dd_rand_num(&groups, 4, 3, &mut output).unwrap();
/// // output: [1, 2, 0] - 选中的参与者索引
/// ```
pub fn get_k_dd_rand_num(
    groups: &[&[u128]],
    n: usize,
    k: usize,
    output: &mut [usize],
) -> Result<()> {
    // 验证输入参数
    if groups.is_empty() || n == 0 || k == 0 {
        return Err(Error::InvalidInput);
    }

    // 检查约束条件：n必须 <= 100,000，k必须 <= 1,000
    if n > 100_000 || k > 1_000 {
        return Err(Error::InvalidInput);
    }

    // 验证n是否为2的n次幂
    if n & (n - 1) != 0 {
        return Err(Error::InvalidInput);
    }

    // 检查k <= n（不能选择比可用参与者更多的参与者）
    if k > n {
        return Err(Error::InvalidInput);
    }

    // 验证groups.len()与n匹配
    if groups.len() != n {
        return Err(Error::InvalidInput);
    }

    // 验证output.len()与k匹配
    if output.len() != k {
        return Err(Error::InvalidInput);
    }

    // 检查所有组都有相同的长度k
    for group in groups {
        if group.len() != k {
            return Err(Error::InvalidInput);
        }
    }

    // 单独处理第一个值
    let n_u128 = n as u128;

    // 计算第一个基础值（使用异或运算）
    let mut result = 0u128;
    for group in groups {
        result ^= group[0];
    }

    // 第一个值直接使用基础值
    let first_offset = (result % n_u128) as usize;
    output[0] = first_offset;

    // 创建简单数组来跟踪已使用的参与者（对于n <= 1000）
    let mut used = [false; 1000];
    if n <= 1000 {
        used[first_offset] = true;

        // 基于之前的结果处理后续值
        for i in 1..k {
            // 计算此位置的基础值（使用异或运算）
            let mut result = 0u128;
            for group in groups {
                result ^= group[i];
            }

            // 以基础值作为偏移量开始
            let mut off_t = (result % n_u128) as usize;

            // 使用直接数组访问找到第一个未使用的偏移量
            while used[off_t] {
                off_t = (off_t + 1) % n;
            }

            // 标记此偏移量为已使用
            used[off_t] = true;

            // 将最终输出值设置为参与者索引
            output[i] = off_t;
        }
    } else {
        // 对于大n，使用原始方法通过检查现有输出条目
        for i in 1..k {
            // 计算此位置的基础值（使用异或运算）
            let mut result = 0u128;
            for group in groups {
                result ^= group[i];
            }

            // 以基础值作为偏移量开始
            let mut off_t = (result % n_u128) as usize;

            // 通过检查现有输出条目找到第一个未使用的偏移量
            loop {
                let mut found = false;
                for j in 0..i {
                    if output[j] == off_t {
                        found = true;
                        break;
                    }
                }

                if !found {
                    break; // 找到未使用的偏移量
                }

                // 尝试下一个偏移量，必要时回绕
                off_t = (off_t + 1) % n;
            }

            // 将最终输出值设置为参与者索引
            output[i] = off_t;
        }
    }

    Ok(())
}

/// 生成k个去中心化随机数，带有排除索引的白名单
///
/// 此函数与`get_k_dd_rand_num`相同，但添加了一个`whitelist`，
/// 包含永远不能被选择的索引。使用异或运算可以保证概率相等，
/// 只要保证n是2的n次幂即可。所有其他行为和约束条件保持不变。
///
/// # 参数
///
/// * `groups` - n组的切片，每组包含k个值
/// * `n` - 组数（应与groups.len()匹配，必须 <= 100,000且必须是2的n次幂）
/// * `k` - 每组的值的数量（应与output.len()匹配，必须 <= 1,000）
/// * `whitelist` - 在[0, n)范围内不能被选择的索引切片
/// * `output` - 用于存储k个选中参与者索引的可变切片
///
/// # 返回值
///
/// * `Result<()>` - 成功时返回Ok，计算失败时返回Error
pub fn get_k_dd_rand_num_with_whitelist(
    groups: &[&[u128]],
    n: usize,
    k: usize,
    whitelist: &[usize],
    output: &mut [usize],
) -> Result<()> {
    // 验证输入参数
    if groups.is_empty() || n == 0 || k == 0 {
        return Err(Error::InvalidInput);
    }

    // 检查约束条件：n必须 <= 100,000，k必须 <= 1,000
    if n > 100_000 || k > 1_000 {
        return Err(Error::InvalidInput);
    }

    // 验证n是否为2的n次幂
    if n & (n - 1) != 0 {
        return Err(Error::InvalidInput);
    }

    // 检查k <= n（不能选择比可用参与者更多的参与者）
    if k > n {
        return Err(Error::InvalidInput);
    }

    // 验证groups.len()与n匹配
    if groups.len() != n {
        return Err(Error::InvalidInput);
    }

    // 验证output.len()与k匹配
    if output.len() != k {
        return Err(Error::InvalidInput);
    }

    // 检查所有组都有相同的长度k
    for group in groups {
        if group.len() != k {
            return Err(Error::InvalidInput);
        }
    }

    // 验证白名单索引在[0, n)范围内
    for &idx in whitelist {
        if idx >= n {
            return Err(Error::InvalidInput);
        }
    }

    let n_u128 = n as u128;

    // 对于n <= 1000，使用小的固定大小已使用数组并预标记白名单
    if n <= 1000 {
        let mut used = [false; 1000];

        // 将白名单索引标记为已使用
        for &w in whitelist {
            used[w] = true;
        }

        // 处理第一个值（使用异或运算）
        let mut result0 = 0u128;
        for group in groups {
            result0 ^= group[0];
        }
        let mut off_t = (result0 % n_u128) as usize;
        let mut tries = 0usize;
        while used[off_t] {
            off_t = (off_t + 1) % n;
            tries += 1;
            if tries >= n {
                return Err(Error::CalculationFailed);
            }
        }
        used[off_t] = true;
        output[0] = off_t;

        // 处理后续值（使用异或运算）
        for i in 1..k {
            let mut result = 0u128;
            for group in groups {
                result ^= group[i];
            }

            let mut off_t = (result % n_u128) as usize;
            let mut tries = 0usize;
            while used[off_t] {
                off_t = (off_t + 1) % n;
                tries += 1;
                if tries >= n {
                    return Err(Error::CalculationFailed);
                }
            }
            used[off_t] = true;
            output[i] = off_t;
        }

        return Ok(());
    }

    // 大n路径：避免大的栈分配；检查白名单和现有输出
    // 处理第一个值（使用异或运算）
    let mut result0 = 0u128;
    for group in groups {
        result0 ^= group[0];
    }
    let mut off_t = (result0 % n_u128) as usize;
    let mut tries = 0usize;
    loop {
        if !is_whitelisted(off_t, whitelist) {
            break;
        }
        off_t = (off_t + 1) % n;
        tries += 1;
        if tries >= n {
            return Err(Error::CalculationFailed);
        }
    }
    output[0] = off_t;

    // 后续值（使用异或运算）
    for i in 1..k {
        let mut result = 0u128;
        for group in groups {
            result ^= group[i];
        }

        let mut off_t = (result % n_u128) as usize;
        let mut tries = 0usize;
        loop {
            let mut found = false;
            // 检查之前的选择
            for j in 0..i {
                if output[j] == off_t {
                    found = true;
                    break;
                }
            }

            // 如果不在白名单中则跳过
            if !found && !is_whitelisted(off_t, whitelist) {
                break; // 找到可接受的偏移量
            }

            off_t = (off_t + 1) % n;
            tries += 1;
            if tries >= n {
                return Err(Error::CalculationFailed);
            }
        }

        output[i] = off_t;
    }

    Ok(())
}
