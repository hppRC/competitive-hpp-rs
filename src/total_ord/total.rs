use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Total<T>(pub T);

impl<T> Total<T>
where
    T: Clone,
{
    pub fn unwrap(&self) -> T {
        self.0.clone()
    }
}
impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_ord() -> () {
        let test = vec![2.2f64, 1.0f64, 8.5f64, 0.5f64, -3.5f64];
        let sorted: Vec<f64> = vec![-3.5f64, 0.5f64, 1.0f64, 2.2f64, 8.5f64];

        let mut total: Vec<Total<f64>> = test.iter().map(|&i| Total(i)).collect();
        total.sort();
        let total: Vec<f64> = total.iter().map(|&Total(i)| i).collect();

        assert_eq!(sorted, total);
    }
}
