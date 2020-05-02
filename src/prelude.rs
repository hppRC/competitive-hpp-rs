// re-export frequently used std items
pub use std::cmp::{max, min, Reverse};
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

pub use itertools::*;
pub use memoise::{memoise, memoise_map};

// input! macro
pub use proconio::marker::*;
pub use proconio::*;

// num stuffs
pub use num::complex::Complex;
pub use num::integer::*;

pub use num::*;

// re-exports
pub use crate::dijkstra::Dijkstra;
pub use crate::mod_fact::ModFact;
pub use crate::union_find::UnionFind;
pub use crate::utils::*;

pub const ALPHABETS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub const DIRE: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub const DIRE8: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
