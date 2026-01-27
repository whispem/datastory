// Unit tests for analysis functions
use datastory::analyze::{
    detect_column_type, mean, median, mode, pearson_correlation, quartiles,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(mean(&data), 2.5);
    }

    #[test]
    fn test_median_even() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(median(&data), 2.5);
    }

    #[test]
    fn test_median_odd() {
        let data = vec![1.0, 2.0, 3.0];
        assert_eq!(median(&data), 2.0);
    }

    #[test]
    fn test_mode() {
        let data = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        assert_eq!(mode(&data), "a");
    }

    #[test]
    fn test_quartiles() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (q1, q3) = quartiles(&data);
        assert_eq!(q1, 2.0);
        assert_eq!(q3, 4.0);
    }

    #[test]
    fn test_detect_column_type_numeric() {
        let data = vec!["1.0".to_string(), "2.0".to_string()];
        assert_eq!(detect_column_type(&data), "numeric");
    }

    #[test]
    fn test_detect_column_type_categorical() {
        let data = vec!["apple".to_string(), "banana".to_string()];
        assert_eq!(detect_column_type(&data), "categorical");
    }

    #[test]
    fn test_pearson_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![2.0, 4.0, 6.0, 8.0];
        let corr = pearson_correlation(&x, &y).unwrap();
        assert!((corr - 1.0).abs() < 1e-6);
    }
}
