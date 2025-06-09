#![allow(dead_code)]
use std::fmt;
use std::collections::HashMap;

#[derive(Eq, Clone, Debug)]
enum Partition {
    Leaf(Leaf),
    Node(Node),
    None,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Leaf {
    cost:usize,
    m:usize,
    n:usize,
}

#[derive(Eq, Clone, Debug)]
struct Node {
    cost:usize,
    transposed:bool,
    left: Box<Partition>,
    right:Box<Partition>,
}


// impl fmt::Debug for Partition {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Partition {{ cost: {} }}", self.cost())
//     }
// }


impl PartialEq for Node {
    // indifferent
    fn eq(&self, other:&Self) -> bool {
        self.cost == other.cost
        && self.transposed == other.transposed
    }
}

impl Leaf {
    fn new(cost:usize, m:usize, n:usize) -> Self {
        Leaf{ cost, m, n }
    }
}

impl Partition {
    fn cost(&self) -> usize {
        match self {
            Partition::Leaf(l) => l.cost,
            Partition::Node(n) => n.cost,
            Partition::None => usize::MAX,
        }
    }
    fn merge(transposed:bool, left:Partition, right:Partition) -> Self {
        if matches!(left, Partition::None) || matches!(right, Partition::None) {
            return Partition::None;
        }
        let cost = left.cost() + right.cost();
        return Partition::Node(Node {
            cost,
            transposed,
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}

impl PartialEq for Partition {
    fn eq(&self, other:&Self) -> bool {
        self.cost() == other.cost()
    }
}

fn tile(i:usize, j:usize, strats:&[Leaf]) -> Partition {
    // let mut memo = vec![vec![None;j];i];
    let mut memo:HashMap<(usize,usize), Partition> = HashMap::new();
    solve(i, j, strats, &mut memo)
}

// fn solve(i:usize, j:usize, strats:&[Leaf], memo:&mut Vec<Vec<Option<Partition>>>) -> Partition {
fn solve(
    i:usize, j:usize,
    strats:&[Leaf],
    memo:&mut HashMap<(usize,usize), Partition>
) -> Partition {
    // if let Some(existing) = &memo[i-1][j-1] {
    if let Some(existing) = memo.get(&(i, j)) {
        return existing.clone()
    }
    for st in strats {
        if st.m == i && st.n == j {
            let result = Partition::Leaf(st.clone());
            // memo[i-1][j-1] = Some(result.clone());
            memo.insert((i,j), result.clone());
            return Partition::Leaf(st.clone())
        }
    }
    let mut best = Partition::None;
    for st in strats {
        if st.m > i || st.n > j {
            continue;
        }
        // i, j are both 1 indexed so strictly less
        if st.m < i {
            let part_above = Partition::merge(
                false,
                solve(st.m, j, strats, memo),
                solve(i - st.m, j, strats, memo),
            );
            if part_above.cost() < best.cost() {
                best = part_above;
            }
        }
        if st.n < j {
            let part_left = Partition::merge(
                true,
                solve(i, j - st.n, strats, memo),
                solve(i, st.n, strats, memo),
            );
            if part_left.cost() < best.cost() {
                best = part_left;
            }
        }
    }
    // memo[i-1][j-1] = Some(best.clone());
    memo.insert((i,j), best.clone());
    best 
}

fn main() {
    let strats = &[
        Leaf::new(2, 1, 1),
        Leaf::new(3, 2, 1),
        Leaf::new(3, 1, 2),
        Leaf::new(4, 2, 2),
    ];

    let dims = &[
        (1,1),
        (1,2),
        (2,1),
        (2,2),
        (4,4),
        (4,2),
        (5, 4),
    ];
    
    for (i, j) in dims {
        println!("-----------------------");
        println!("Solving (i: {},j: {})", i, j);
        let result = tile(*i, *j, strats);
        println!("-----------------------");
        println!("-----------------------");
        println!("{:?}", result);
    }
    // let p = Partition::Leaf(Leaf::new(2, 1, 1));
    // let mut x = Partition::merge(false, p.clone(), p.clone());
    // let mut y = Partition::merge(false, x.clone(), x.clone());
    // println!("X {:?}", x);
    // println!("Y {:?}", y);
    // println!("---------");
    // let mut y = Partition::merge(false, x.clone(), Partition::None);
    // println!("X {:?}", x);
    // println!("Y {:?}", y);

}
