pub mod searching;
pub mod sorting;

use searching::*;
#[cfg(test)]
mod tests {
    use crate::searching::linear_search;

    #[test]

    fn test_linear() {
        let mut numbers: Vec<i32> = vec![9, 8, 7, 6, 5, 5, 4, 33, 2, 1, 3];
        assert_eq!(4, linear_search(5, &mut numbers).unwrap());
    }
}
