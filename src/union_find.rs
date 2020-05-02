/// # UnionFind
///
/// Example:
/// ```
/// use competitive_hpp::union_find::UnionFind;
///
/// let mut uf = UnionFind::new(5);
/// uf.union(0, 1);
/// uf.union(2, 3);
/// uf.union(1, 4);
///
/// uf.find(1); // 0
/// uf.is_same(0, 1); // true
/// uf.group_size(0); // 3
/// ```
#[derive(Clone, Debug)]
pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    group: Vec<usize>,
}

impl UnionFind {
    /// ```
    /// let mut uf = UnionFind::new(5);
    /// ```
    pub fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
            group: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let px = self.par[x];
            let root = self.find(px);
            // reattach edges
            self.par[x] = root;
            root
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.group[y] += self.group[x];
            self.par[x] = y;
        } else {
            self.group[x] += self.group[y];
            self.par[y] = x;
        }
        if self.rank[x] == self.rank[y] {
            self.rank[y] += 1;
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn group_size(&mut self, x: usize) -> usize {
        let p = self.find(x);
        self.group[p]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn union_find_test() {
        let mut uf = UnionFind::new(5);

        // 0 ━━━━━ 1 ━━━━━ 4
        //
        // 2 ━━━━━ 3

        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(1, 4);

        assert_eq!(uf.find(0), uf.find(1));
        assert_ne!(uf.find(0), uf.find(2));
        assert_ne!(uf.find(0), uf.find(3));
        assert_eq!(uf.find(0), uf.find(4));
        assert_ne!(uf.find(1), uf.find(2));
        assert_ne!(uf.find(1), uf.find(3));
        assert_eq!(uf.find(1), uf.find(4));
        assert_eq!(uf.find(2), uf.find(3));
        assert_ne!(uf.find(2), uf.find(4));
        assert_ne!(uf.find(3), uf.find(4));

        assert!(uf.is_same(0, 1));
        assert!(!uf.is_same(0, 2));
        assert!(!uf.is_same(0, 3));
        assert!(uf.is_same(0, 4));

        assert_eq!(uf.rank[0], 0);
        assert_eq!(uf.rank[1], 1);
        assert_eq!(uf.rank[2], 0);
        assert_eq!(uf.rank[3], 1);
        assert_eq!(uf.rank[4], 1);

        assert_eq!(uf.group_size(0), 3);
        assert_eq!(uf.group_size(1), 3);
        assert_eq!(uf.group_size(2), 2);
        assert_eq!(uf.group_size(3), 2);
        assert_eq!(uf.group_size(4), 3);

        // dbg!(uf);
    }
}
