use num::traits::{NumAssignOps, PrimInt, Unsigned};

/// # UnionFind
///
/// Example:
/// ```
/// use competitive_hpp::prelude::*;
/// // 0 ━━━━━ 1 ━━━━━ 4
/// //
/// // 2 ━━━━━ 3
/// let mut uf = UnionFind::new(5usize);
/// uf.union(0, 1);
/// uf.union(2, 3);
/// uf.union(1, 4);
///
/// assert_eq!(uf.find(0), uf.find(1));
/// assert_eq!(2, uf.num_of_groups());
/// assert!(uf.is_same(0, 1));
/// assert_eq!(uf.rank(2), 0);
/// assert_eq!(uf.group_size(0), 3);
/// ```
#[derive(Clone, Debug)]
pub struct UnionFind<T> {
    par: Vec<T>,
    rank: Vec<T>,
    group: Vec<T>,
    num_of_groups: T,
}

impl<T> UnionFind<T>
where
    T: PrimInt + NumAssignOps + Unsigned,
{
    pub fn new(n: T) -> Self {
        let mut par: Vec<T> = vec![];
        let un = n.to_u64().unwrap();
        for i in 0..un {
            par.push(T::from(i).unwrap())
        }

        UnionFind {
            par,
            rank: vec![T::zero(); n.to_usize().unwrap()],
            group: vec![T::one(); n.to_usize().unwrap()],
            num_of_groups: n,
        }
    }

    pub fn find(&mut self, x: T) -> T {
        let ux = x.to_usize().unwrap();
        if self.par[ux] == x {
            x
        } else {
            let px = self.par[ux];
            let root = self.find(px);
            // reattach edges
            self.par[ux] = root;
            root
        }
    }

    pub fn union(&mut self, x: T, y: T) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        let ux = x.to_usize().unwrap();
        let uy = y.to_usize().unwrap();

        if self.rank[ux] < self.rank[uy] {
            let tmp = self.group[ux];
            self.group[uy] += tmp;
            self.par[ux] = y;
        } else {
            let tmp = self.group[uy];
            self.group[ux] += tmp;
            self.par[uy] = x;
        }
        if self.rank[ux] == self.rank[uy] {
            self.rank[uy] += T::one();
        }
        self.num_of_groups -= T::one();
    }

    pub fn is_same(&mut self, x: T, y: T) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn group_size(&mut self, x: T) -> T {
        let p = self.find(x);
        self.group[p.to_usize().unwrap()]
    }

    pub fn rank(&self, x: T) -> T {
        self.rank[x.to_usize().unwrap()]
    }

    pub fn num_of_groups(&self) -> T {
        self.num_of_groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! impl_union_find_tests {
        ($ty:ty) => {
            let mut uf = UnionFind::new(5 as $ty);

            // 0 ━━━━━ 1 ━━━━━ 4
            //
            // 2 ━━━━━ 3
            assert_eq!(5, uf.num_of_groups());

            uf.union(0, 1);
            assert_eq!(4, uf.num_of_groups());

            uf.union(2, 3);
            assert_eq!(3, uf.num_of_groups());

            uf.union(1, 4);
            assert_eq!(2, uf.num_of_groups());

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

            assert_eq!(uf.rank(0), 0);
            assert_eq!(uf.rank(1), 1);
            assert_eq!(uf.rank(2), 0);
            assert_eq!(uf.rank(3), 1);
            assert_eq!(uf.rank(4), 1);

            assert_eq!(uf.group_size(0), 3);
            assert_eq!(uf.group_size(1), 3);
            assert_eq!(uf.group_size(2), 2);
            assert_eq!(uf.group_size(3), 2);
            assert_eq!(uf.group_size(4), 3);

            uf.union(0, 2);
            assert_eq!(1, uf.num_of_groups());
        };
    }

    #[test]
    fn u64_union_find_test() {
        impl_union_find_tests!(u64);
    }
    #[test]
    fn u32_union_find_test() {
        impl_union_find_tests!(u32);
    }
    #[test]
    fn usize_union_find_test() {
        impl_union_find_tests!(usize);
    }
}
