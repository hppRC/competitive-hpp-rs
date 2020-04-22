use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// # Dijkstra
///
/// Example:
/// ```
/// use competitive_hpp::dijkstra::Dijkstra;
///
/// let edges: Vec<(usize, usize, i64)> = vec![(0, 1, 1),(0, 2, 6),(1, 3, 2)];
///
/// //Dijkstra::new(vertex num, edges, start vertex)
/// let dijkstra = Dijkstra::new(4, &edges, 0);
/// ```

#[derive(Clone, Debug)]
pub struct Dijkstra {
    dist: Vec<i64>,
    adjacency_list: Vec<Vec<(usize, i64)>>,
    n: usize,
}

impl Dijkstra {
    pub fn new(n: usize, edges: &Vec<(usize, usize, i64)>, start: usize) -> Self {
        let inf = i64::max_value();

        let mut dist: Vec<i64> = vec![inf; n];
        let adjacency_list = Self::create_adjacency_list(n, &edges);

        // MinHeap
        let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        dist[start] = 0;
        heap.push(Reverse((0, start)));

        while !heap.is_empty() {
            let Reverse((d, v)) = heap.pop().unwrap();

            if dist[v] < d {
                continue;
            }

            for &(u, cost) in &adjacency_list[v] {
                if dist[u] > dist[v] + cost {
                    dist[u] = dist[v] + cost;
                    heap.push(Reverse((dist[u], u)));
                }
            }
        }

        Dijkstra {
            dist,
            adjacency_list,
            n,
        }
    }

    fn create_adjacency_list(n: usize, edges: &Vec<(usize, usize, i64)>) -> Vec<Vec<(usize, i64)>> {
        let mut adjacency_list: Vec<Vec<(usize, i64)>> = vec![vec![]; n];

        for &(a, b, c) in edges {
            adjacency_list[a].push((b, c))
        }

        adjacency_list
    }
}

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
