pub trait VecUtils<T> {
    fn power_set(&self) -> Vec<Vec<T>>;
}

impl<T> VecUtils<T> for Vec<T>
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
                    .collect::<Vec<T>>()
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
}
