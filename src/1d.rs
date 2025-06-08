#![allow(dead_code)]
use std::cmp::Ordering;

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

#[derive(Clone, Eq)]
struct Partition {
    n:usize,
    cost:usize,
    strats:Vec<Strategy>,
}

impl Partition {
    fn new() -> Self {
        Partition { n:0, cost:0, strats: vec![] }
    }
    fn new_max() -> Self {
        Partition { n:0, cost:usize::MAX, strats: vec![] }
    }
    fn append(&mut self, strat:Strategy) {
        self.n += strat.n;
        self.cost += strat.cost;
        self.strats.push(strat);
    }
}

impl PartialEq for Partition {
    fn eq(&self, other:&Self) -> bool {
        self.n == other.n
            && self.cost == other.cost
    }
}

fn tiling(n:usize, strats:&[Strategy]) -> Vec<usize> {
    // returns reverse strategy
    let mut dp:Vec<(usize, usize, usize)> = vec![(0,0,0); n+1];
    let mut result:Vec<usize> = Vec::new();
    for i in 1..=n {
        for (j, s) in strats.iter().enumerate() {
            if i >= s.n {
                let (prev_cost, _, _) = dp[i-s.n];
                let (curr_cost, _, _) = dp[i];
                if prev_cost + s.cost < curr_cost || curr_cost == 0 {
                    dp[i] = (prev_cost + s.cost, i - s.n, j);
                }
            }
        }
    }
    let mut idx = n;
    while idx != 0 {
        let (_, pre_idx, s_idx) = dp[idx];
        result.push(s_idx);
        idx = pre_idx;
    }
    result
}

fn main() {
    println!("hello wrold");
}
