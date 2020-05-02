use crate::total_ord::Total;
use num::traits::Float;

pub trait TotalSort {
    fn total_sort(&mut self) -> ();
}

impl<T> TotalSort for Vec<T>
where
    T: PartialOrd + Float,
{
    fn total_sort(&mut self) -> () {
        let mut tmp: Vec<Total<T>> = self.iter().map(|&i| Total(i)).collect();
        tmp.sort();
        *self = tmp.iter().map(|&Total(i)| i).collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_sort() -> () {
        let mut test = vec![2.2f64, 1.0f64, 8.5f64, 0.5f64, -3.5f64];
        let sorted: Vec<f64> = vec![-3.5f64, 0.5f64, 1.0f64, 2.2f64, 8.5f64];
        test.total_sort();

        assert_eq!(sorted, test);
    }
}
