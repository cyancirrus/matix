#![allow(dead_code)]

#[derive(Eq, PartialEq, Clone, Debug)]
struct MatrixStrategy {
    dims:(usize,usize),
    cost:usize,
}


#[derive(Eq, PartialEq, Clone)]
struct Strategy {
    n:usize,
    cost:usize,
}

impl Strategy {
    fn new(n:usize, cost:usize) -> Self {
        Self { n, cost }
    }
}
// fn tiling_1d(n:usize, strats:&[Strategy]) -> Vec<usize> {
//     // returns reverse strategy
//     let mut dp:Vec<(usize, usize, usize)> = vec![(0,0,0); n+1];
//     let mut result:Vec<usize> = Vec::new();
//     for i in 1..=n {
//         for (j, s) in strats.iter().enumerate() {
//             if i >= s.n {
//                 let (prev_cost, _, _) = dp[i-s.n];
//                 let (curr_cost, _, _) = dp[i];
//                 if prev_cost + s.cost < curr_cost || curr_cost == 0 {
//                     dp[i] = (prev_cost + s.cost, i - s.n, j);
//                 }
//             }
//         }
//     }
//     let mut idx = n;
//     loop {
//         let (_, pre_idx, s_idx) = dp[idx];
//         result.push(s_idx);
//         idx = pre_idx;
//     }
//     result
// }

fn tiling(dims:(usize, usize), strats:&[MatrixStrategy]) -> Vec<usize> {
    // returns reverse strategy
    let (m, n) = dims;

    let mut dp:Vec<(usize, usize, usize)> = vec![(0,0,0); m*n + 1];
    let mut result:Vec<usize> = Vec::new();
    for idx in 1..=m*n {
        for (k, mat) in strats.iter().enumerate() {
            println!("-----------------------");
            println!("Strat {:?}", mat);

            let (mat_i, mat_j) = mat.dims;
            let m_idx = (mat_i - 1)*n + mat_j ; 
            let _i = (m_idx-1) / n + 1;
            let _j = (m_idx-1) % n + 1;
            // one based (i,j) for debug
            println!("idx {}, m_idx {}", idx, m_idx);
            if idx >= m_idx  {
                println!("Mat ({},{}) translated to Tiling ({}, {})", mat_i, mat_j, _i, _j);
                println!("Cur ({}, {}) and inferred Prev ({},{})", mat_i, mat_j, _i, _j);
                let _pre = idx - m_idx;
                println!("preidx {:?}", _pre);
                let _p_i = (_pre ) / n + 1;
                let _p_j = (_pre ) % n + 1;
                let _c_i = (idx - 1) / n + 1;
                let _c_j = (idx - 1) % n + 1;

                let (prev_cost, _, _) = dp[idx-m_idx];
                let (curr_cost, _, _) = dp[idx];
                if prev_cost + mat.cost < curr_cost || curr_cost == 0 {
                    dp[idx] = (prev_cost + mat.cost, idx - m_idx, k);
                }
            }
        }
    }
    let mut idx = n+1;
    println!("dp {:?}", dp);
    while idx != 0 {
        println!("----");
        let (_, pre_idx, s_idx) = dp[idx];
        println!("Dp {:?}", dp[idx]);
        result.push(s_idx);
        idx = pre_idx;
    }
    println!("result {:?}", result);
    result
}

fn print_strat_sequence(strats: &[MatrixStrategy], seq: &[usize]) {
    for &idx in seq.iter().rev() {
        let s = &strats[idx];
        println!("Used: {}x{} (cost {})", s.dims.0, s.dims.1, s.cost);
    }
}

fn test_tiling() {
    let strats = vec![
        MatrixStrategy { dims: (1, 1), cost: 2 },
        MatrixStrategy { dims: (1, 2), cost: 3 },
        MatrixStrategy { dims: (2, 1), cost: 3 },
        // MatrixStrategy { dims: (2, 2), cost: 4 },
    ];

    let dims_list = vec![
        // (1, 1),
        (2, 1),
        // (2, 2),
        // (2, 3),
        // (3, 3),
        // (3, 4),
    ];

    for dims in dims_list {
        println!("\nTesting matrix {}x{}...", dims.0, dims.1);
        let seq = tiling(dims, &strats);
        print_strat_sequence(&strats, &seq);
    }
}



fn main() {
    // println!("hello wrold");
    test_tiling()
}
