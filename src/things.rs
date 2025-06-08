
dp[i, j, k] = 
    if (i, j, k) in precomputed_algorithms:
        cost(precomputed_algorithms[i, j, k])
    else:
        min(
            dp[i/2, j, k] * 2,
            dp[i, j/2, k] * 2,
            dp[i, j, k/2] * 2
        )


fn round_to_base(value: f64, base: f64) -> f64 {
    ((value * base + base / 2.0).floor() / base) * base
}

fn normalize_dims(i: usize, j: usize, k: usize, precision: usize) -> (usize, usize, usize) {
    let sum = (i + j + k) as f64;
    let scale = precision as f64;

    let i_norm = ((i as f64 / sum) * scale).round() as usize;
    let j_norm = ((j as f64 / sum) * scale).round() as usize;
    let k_norm = ((k as f64 / sum) * scale).round() as usize;

    (i_norm, j_norm, k_norm)
}


fn normalize_and_round(i: usize, j: usize, k: usize, base: f64) -> (f64, f64, f64) {
    let i_f = i as f64;
    let j_f = j as f64;
    let k_f = k as f64;

    let _i = i_f / (i_f + j_f);
    let _j = j_f / (i_f + k_f);
    let _k = k_f / (i_f + j_f);

    (
        round_to_base(_i, base),
        round_to_base(_j, base),
        round_to_base(_k, base),
    )
}


fn main() {
    let mut matrix = vec![
        vec![1., 2., 3., 4.],
        vec![5., 6., 7., 8.],
        vec![9., 10., 11., 12.],
        vec![13., 14., 15., 16.],
    ];

    let block_rows = 2;
    let block_cols = 2;

    // Partition rows into blocks
    let row_blocks = matrix.chunks_mut(block_rows);

    for (block_row_idx, row_block) in row_blocks.enumerate() {
        // For each row block, partition each row into column blocks
        for row in row_block.iter_mut() {
            let col_blocks = row.chunks_mut(block_cols);

            for (block_col_idx, col_block) in col_blocks.enumerate() {
                // Here you get a mutable slice for a block: col_block
                // Run your computation here - dummy print for demo
                println!(
                    "Block at row block {}, col block {}: {:?}",
                    block_row_idx, block_col_idx, col_block
                );

                // Example modification: increment each element by 1
                for val in col_block.iter_mut() {
                    *val += 1.0;
                }
            }
        }
    }

    // After partitioning and computation
    println!("Modified matrix:");
    for row in &matrix {
        println!("{:?}", row);
    }
}
