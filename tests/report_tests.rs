// Unit tests for report generation (mocked data)
use datastory::report::generate_html_report;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_html_report_basic() {
        let num_columns = vec![vec![1.0, 2.0, 3.0]];
        let cat_columns = vec![vec!["a".to_string(), "b".to_string()]];
        let headers = vec!["num_col".to_string(), "cat_col".to_string()];
        let result = generate_html_report(&num_columns[..], &cat_columns[..], &headers[..], "test.csv", "report.html");
        assert!(result.is_ok());
    }
}
