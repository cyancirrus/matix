# Matix

Matix is a Rust library for matrix partitioning aimed at optimizing matrix operations by precomputing efficient partitions. It decomposes matrix dimensions into smaller blocks (partitions) using a set of predefined leaf strategies with associated costs, enabling optimized tiled operations.

## Features

- Recursive partitioning of matrix dimensions into sub-blocks
- Memoization to avoid redundant computation
- Supports transposed partitions in merge operations
- Precomputes cost-based optimal partitions from a set of leaf building blocks

## Current Status

- The core algorithm for partitioning matrix dimensions without padding is fully implemented and working.
- Supports partition merges with transposition flags to represent orientation.
- Memoized dynamic programming approach ensures efficient recomputation avoidance.

## Planned / Upcoming

- Implementation of new algorithms such as Strassens and Winograd as well as other recent algorithmic improvements.
- Support for matrix padding cases to enable more general tiled partitioning.

## How it Works

The library represents partitions as either:

- **Leaf**: Base partition unit with a fixed cost and dimensions (`m` x `n`)
- **Node**: Combination of two partitions (left and right), possibly with a transpose flag, accumulating cost
- **None**: Represents an invalid or non-existent partition (used for pruning)

The main algorithm tries to build the best partition for a given dimension `(i, j)` by:

1. Checking if the exact dimension matches any leaf partition
2. Recursively splitting the dimension horizontally or vertically based on the leaves and combining subpartitions
3. Selecting the partition with minimal total cost
4. Using memoization (`HashMap`) to cache and reuse computed partitions for sub-dimensions

## Example Usage

```rust
fn main() {
    let strats = &[
        Leaf::new(2, 1, 1),
        Leaf::new(3, 2, 1),
        Leaf::new(3, 1, 2),
        Leaf::new(4, 2, 2),
    ];

    let dims = &[(1,1), (1,2), (2,1), (2,2), (4,4), (4,2), (5,4)];
    
    for (i, j) in dims {
        println!("Solving (i: {}, j: {})", i, j);
        let result = tile(*i, *j, strats);
        println!("{:?}", result);
    }
}
```
