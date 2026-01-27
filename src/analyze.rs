/// Automatically detect column type: numeric, categorical, boolean, date
pub fn detect_column_type(col: &[String]) -> &'static str {
    let is_bool = col
        .iter()
        .all(|v| v == "true" || v == "false" || v == "True" || v == "False");
    if is_bool {
        return "boolean";
    }
    let is_numeric = col.iter().all(|v| v.parse::<f64>().is_ok());
    if is_numeric {
        return "numeric";
    }
    let is_date = col
        .iter()
        .all(|v| chrono::NaiveDate::parse_from_str(v, "%Y-%m-%d").is_ok());
    if is_date {
        return "date";
    }
    "categorical"
}
/// Compute Pearson correlation between two numeric columns
pub fn pearson_correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.is_empty() {
        return None;
    }
    let mean_x = mean(x);
    let mean_y = mean(y);
    let numerator: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(a, b)| (a - mean_x) * (b - mean_y))
        .sum();
    let denominator_x: f64 = x.iter().map(|a| (a - mean_x).powi(2)).sum();
    let denominator_y: f64 = y.iter().map(|b| (b - mean_y).powi(2)).sum();
    let denominator = (denominator_x * denominator_y).sqrt();
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

/// Suggest cleaning actions for a numeric column
pub fn cleaning_suggestions(col: &[f64]) -> Vec<String> {
    let mut suggestions = Vec::new();
    let missing = col.iter().filter(|v| v.is_nan()).count();
    if missing > 0 {
        suggestions.push(format!("{} missing values detected", missing));
    }
    let (q1, q3) = quartiles(col);
    let iqr = q3 - q1;
    let lower = q1 - 1.5 * iqr;
    let upper = q3 + 1.5 * iqr;
    let outliers = col.iter().filter(|v| **v < lower || **v > upper).count();
    if outliers > 0 {
        suggestions.push(format!("{} outliers detected", outliers));
    }
    if suggestions.is_empty() {
        suggestions.push("No cleaning needed".to_string());
    }
    suggestions
}
// Analysis and statistics functions

pub fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / (data.len() as f64)
}

pub fn median(data: &[f64]) -> f64 {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted.len();
    if len.is_multiple_of(2) {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    } else {
        sorted[len / 2]
    }
}

pub fn mode(data: &[String]) -> String {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for val in data {
        *counts.entry(val).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(val, _)| val.clone())
        .unwrap_or_default()
}

pub fn quartiles(data: &[f64]) -> (f64, f64) {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted.len();
    let q1_idx = len as f64 * 0.25;
    let q3_idx = len as f64 * 0.75;
    let q1 = sorted[q1_idx.floor() as usize];
    let q3 = sorted[q3_idx.floor() as usize];
    (q1, q3)
}
