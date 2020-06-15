use std::collections::{BTreeMap, BTreeSet};

pub trait BTreeMapUtils<T, U> {
    fn min_elem(&self) -> Option<(&T, &U)>;
    fn max_elem(&self) -> Option<(&T, &U)>;
}

impl<T, U> BTreeMapUtils<T, U> for BTreeMap<T, U> {
    fn min_elem(&self) -> Option<(&T, &U)> {
        self.iter().next()
    }
    fn max_elem(&self) -> Option<(&T, &U)> {
        self.iter().next_back()
    }
}

pub trait BTreeSetUtils<T> {
    fn min_elem(&self) -> Option<&T>;
    fn max_elem(&self) -> Option<&T>;
}

impl<T> BTreeSetUtils<T> for BTreeSet<T> {
    fn min_elem(&self) -> Option<&T> {
        self.iter().next()
    }
    fn max_elem(&self) -> Option<&T> {
        self.iter().next_back()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn btree_map_test() {
        let tree: BTreeMap<i32, i32> = maplit::btreemap! { 1 => 1, 3 => 3, 2 => 2 };

        assert_eq!(tree.min_elem(), Some((&1, &1)));
        assert_eq!(tree.max_elem(), Some((&3, &3)));
    }

    #[test]
    fn btree_set_test() {
        let set: BTreeSet<i32> = maplit::btreeset! { 1, 3, 2 };

        assert_eq!(set.min_elem(), Some(&1));
        assert_eq!(set.max_elem(), Some(&3));
    }
}
