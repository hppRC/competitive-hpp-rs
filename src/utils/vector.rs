trait VecUtils<T> {
    fn transpose(&self) -> Vec<Vec<T>>;
}

impl<T> VecUtils<T> for Vec<Vec<T>>
where
    T: Clone,
{
    fn transpose(&self) -> Vec<Vec<T>> {
        (0..self[0].len())
            .map(|j| {
                (0..self.len())
                    .map(|i| self[i][j].clone())
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_test() {
        let test = vec![vec![0, 1, 2, 3]; 2];

        assert_eq!(
            vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3],],
            test.transpose()
        )
    }
}
