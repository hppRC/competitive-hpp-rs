use crate::total_ord::Total;
use num::traits::{Bounded, Num, PrimInt};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// # Dijkstra
///
/// Example:
/// ```
/// use competitive_hpp::prelude::*;
/// // edge: Vec<(from, to, cost)>
/// let edges = vec![(0, 1, 1),(0, 2, 6),(1, 3, 2)];
///
/// //Dijkstra::new(vertex num, edges, start vertex)
/// let dijkstra = Dijkstra::new(4, &edges, 0);
/// assert_eq!(dijkstra.dist[0], 0);
/// assert_eq!(dijkstra.dist[1], 1);
/// assert_eq!(dijkstra.dist[2], 6);
/// ```
#[derive(Clone, Debug)]
pub struct Dijkstra<T, F>
where
    T: PrimInt,
    F: Num + Bounded + Clone + Copy + PartialOrd,
{
    pub dist: Vec<F>,
    pub adjacency_list: Vec<Vec<(usize, F)>>,
    n: T,
}

impl<T, F> Dijkstra<T, F>
where
    T: PrimInt,
    F: Num + Bounded + Clone + Copy + PartialOrd,
{
    pub fn new(n: T, edges: &[(usize, usize, F)], start: usize) -> Self {
        let inf = F::max_value();

        let mut dist: Vec<F> = vec![inf; n.to_usize().unwrap()];
        let adjacency_list = Self::create_adjacency_list(n, &edges);

        // MinHeap
        let mut heap: BinaryHeap<Total<Reverse<(F, usize)>>> = BinaryHeap::new();

        dist[start] = F::zero();
        heap.push(Total(Reverse((F::zero(), start))));

        while !heap.is_empty() {
            let Total(Reverse((d, v))) = heap.pop().unwrap();

            if dist[v] < d {
                continue;
            }

            for &(u, cost) in adjacency_list[v].iter() {
                if dist[u] > dist[v] + cost {
                    dist[u] = dist[v] + cost;
                    heap.push(Total(Reverse((dist[u], u))));
                }
            }
        }

        Dijkstra {
            dist,
            adjacency_list,
            n,
        }
    }

    fn create_adjacency_list(n: T, edges: &[(usize, usize, F)]) -> Vec<Vec<(usize, F)>> {
        let mut adjacency_list: Vec<Vec<(usize, F)>> = vec![vec![]; n.to_usize().unwrap()];

        for &(from, to, cost) in edges {
            adjacency_list[from].push((to, cost))
        }

        adjacency_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dijkstra_test() {
        // | -------2------->|
        // 0 --1--> 1 --2--> 3 --|
        // |---6--> 2 --2--->|   |
        // |<---------4----------|
        let edges = vec![
            (0, 1, 1),
            (0, 2, 6),
            (1, 3, 2),
            (2, 3, 2),
            (0, 3, 2),
            (3, 0, 4),
        ];

        let dijkstra = Dijkstra::new(4, &edges, 0);

        assert_eq!(dijkstra.dist[0], 0);
        assert_eq!(dijkstra.dist[1], 1);
        assert_eq!(dijkstra.dist[2], 6);
        assert_eq!(dijkstra.dist[3], 2);

        let dijkstra_another = Dijkstra::new(4, &edges, 1);

        assert_eq!(dijkstra_another.dist[0], 6);
        assert_eq!(dijkstra_another.dist[1], 0);
        assert_eq!(dijkstra_another.dist[2], 12);
        assert_eq!(dijkstra_another.dist[3], 2);
    }

    #[test]
    fn float_dijkstra_test() {
        // 0 --1.5--> 1 --6.2--> 2
        // |---------4.3-------->|
        let float_edges = vec![(0, 1, 1.5f64), (1, 2, 6.2f64), (0, 2, 4.3f64)];

        let float_dijkstra = Dijkstra::new(3, &float_edges, 0);

        assert_eq!(float_dijkstra.dist[0], 0f64);
        assert_eq!(float_dijkstra.dist[1], 1.5);
        assert_eq!(float_dijkstra.dist[2], 4.3);
    }
}
