use std::collections::HashSet;

pub trait PowerSetTrait<T> {
    fn power_set(&self) -> Vec<Vec<T>>;
}

impl<T> PowerSetTrait<T> for Vec<T>
where
    T: Clone,
{
    fn power_set(&self) -> Vec<Vec<T>> {
        let n = self.len();
        (0..1 << n)
            .map(|pattern| {
                (0..n)
                    .filter_map(|i| {
                        if (pattern >> i) & 1 == 1 {
                            Some(self[i].clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl<K> PowerSetTrait<K> for HashSet<K>
where
    K: Clone,
{
    /// Unspecified order
    fn power_set(&self) -> Vec<Vec<K>> {
        let n = self.len();
        let v: Vec<_> = self.iter().collect();
        (0..1 << n)
            .map(|pattern| {
                (0..n)
                    .filter_map(|i| {
                        if (pattern >> i) & 1 == 1 {
                            Some(v[i].clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_test() {
        let mut test = vec![0, 1, 2];

        assert_eq!(
            vec![
                vec![],
                vec![0],
                vec![1],
                vec![0, 1],
                vec![2],
                vec![0, 2],
                vec![1, 2],
                vec![0, 1, 2]
            ],
            test.power_set()
        );

        test.push(3);
        test.push(4);

        assert_eq!(test.power_set().len(), 32);
    }

    #[test]
    fn set_test() {
        let test = maplit::hashset! { 0, 1, 2 };

        assert_eq!(
            vec![
                vec![],
                vec![0],
                vec![1],
                vec![0, 1],
                vec![2],
                vec![0, 2],
                vec![1, 2],
                vec![0, 1, 2]
            ]
            .len(),
            test.power_set().len()
        );

        let test_set = maplit::hashset! {
            vec![],
            vec![0],
            vec![1],
            vec![0, 1],
            vec![2],
            vec![0, 2],
            vec![1, 2],
            vec![0, 1, 2]
        };
        let test_power_set: HashSet<Vec<_>> = test.power_set().into_iter().collect();

        assert_eq!(test_set.len(), test_power_set.len());
        assert!(test_power_set.contains(&vec![0,]));
    }
}
