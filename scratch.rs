#![allow(dead_code)]
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Debug)]
struct MatrixStrategy {
    cost:usize,
    m:usize,
    n:usize,
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
            let m_i = (m_idx-1) / n ;
            let m_j = (m_idx-1) % n ;
            // one based (i,j) for debug
            println!("idx {}, m_idx {}", idx, m_idx);
            if idx >= m_idx  && mat_i <= m && mat_j <= n {
                println!("Mat ({},{}) translated to Tiling ({}, {})", mat_i, mat_j, m_i, m_j);
                // println!("Cur ({}, {}) and inferred Prev ({},{})", mat_i, mat_j, _i, _j);
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
    for i in 0..m {
        let idx = n*i+1;
        println!("{:?}", &dp[idx..idx+n]);
    }
    let mut idx = m*n;
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
        MatrixStrategy { dims: (2, 2), cost: 4 },
    ];

    let dims_list = vec![
        // (1, 1),
        // (2, 1),
        // (2, 2),
        // (2, 3),
        // (3, 3),
        // (3, 4),
        // (4, 4),
        (2, 4),
    ];

    for dims in dims_list {
        println!("\nTesting matrix {}x{}...", dims.0, dims.1);
        let seq = tiling(dims, &strats);
        print_strat_sequence(&strats, &seq);
    }
}

type Strategems = Vec<(Option<Partition>, Option<Partition>)>; 

#[derive(Eq)]
struct Partition {
    trans:bool,
    cost:usize,
    strat:MatrixStrategy,
    strategems:Strategems,
}

impl Partition {
    fn new(trans:bool, cost:usize, strategy:MatrixStrategy, strategems:Strategems) -> Self {
        Self { cost: base.cost, strategems:vec![base] }
    }
    fn compose(trans:bool, base:Partition, other:Partition) -> Self {
        let cost = base.cost + other.cost;
        let strategems = vec![(base,other)];
        Self { cost, strategems }
    }
}

impl PartialEq for Partition {
    fn eq(&self, other:&Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for Partition {
    fn cmp(&self, other:&Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Partition {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// fn tile(n, strats) -> Strats {
//     solve(n, strats)
// }

fn solve(i:usize, j:usize, strats:&[MatrixStrategy]) -> Partition {
    // i, j, cost, strat
    let mut dp: Vec<Vec<Partition>> = Vec::with_capacity(i * j);
    for (s, st) in strats.iter().enumerate() {
        if st.m <= i && st.n <= j {
            let 
            let part_above = Partition::init(
                trans
                
                Partition::init(
                    solve(i - st.m, j, strats),
                    solve(st.n, j, strats),
                ).min(
                    Partition::init(
                        solve(i - st.m, j, strats),
                        solve(st.n, j, strats),
                    ),
                )

        }
        dp[i][j] = dp[i][j].min(
        )
        // dp[i][j] = dp[i][j].min(
        //     Partition::init(
        //         solve(i -strats.i, j),
        //         solve(strats.i, j),
        //     ),
        //     Partition::init(
        //         solve(i, j - strats.j),
        //         solve(i, strats.j),
        //     ),
        // )
    }

    todo!()

    
}





fn main() {
    // println!("hello wrold");
    test_tiling()
}
