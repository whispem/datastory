#![allow(deprecated, unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]
use pyo3::prelude::*;
use std::env;
mod analyze;
mod formats;
mod report;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <data_file> [output_report.html]", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let report_name = if args.len() > 2 {
        &args[2]
    } else {
        "rapport.html"
    };
    if let Err(e) = analyze_csv_with_report(filename, report_name) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    #[pyfunction]
    fn analyze_csv_py(path: &str) -> PyResult<String> {
        match analyze_csv_with_report(path, "rapport.html") {
            Ok(_) => Ok("Report generated successfully".to_string()),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
        }
    }

    #[pymodule]
    fn datastory(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(analyze_csv_py, m)?)?;
        Ok(())
    }
    fn analyze_csv_with_report(path: &str, report_name: &str) -> Result<(), Box<dyn Error>> {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let data = match ext {
            "csv" => formats::read_csv(path)?,
            "parquet" => formats::read_parquet(path)?,
            "json" => formats::read_json(path)?,
            _ => return Err("Unsupported file format".into()),
        };
        // Split numeric and categorical columns
        let col_count = if !data.is_empty() { data[0].len() } else { 0 };
        let mut num_columns: Vec<Vec<f64>> = vec![vec![]; col_count];
        let mut cat_columns: Vec<Vec<String>> = vec![vec![]; col_count];
        for row in &data {
            for (i, value) in row.iter().enumerate() {
                if let Ok(num) = value.parse::<f64>() {
                    num_columns[i].push(num);
                } else {
                    cat_columns[i].push(value.to_string());
                }
            }
        }
        let headers: Vec<String> = (0..col_count)
            .map(|i| format!("Column_{}", i + 1))
            .collect();

        // Generate HTML report
        report::generate_html_report(
            &num_columns,
            &cat_columns,
            &headers,
            path,
            report_name,
        )?;
        println!("HTML report generated: {}", report_name);
        Ok(())
    }

    fn save_histogram(
        filename: &str,
        data: &[f64],
        colname: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use plotters::prelude::*;
        let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Histogram - {}", colname), ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(min..max, 0..data.len() as u32)?;
        chart.configure_mesh().draw()?;
        let bin_count = 20;
        let bin_width = (max - min) / bin_count as f64;
        let mut bins = vec![0u32; bin_count];
        for &v in data {
            let idx = ((v - min) / bin_width).floor() as usize;
            let idx = if idx >= bin_count { bin_count - 1 } else { idx };
            bins[idx] += 1;
        }
        chart.draw_series(bins.iter().enumerate().map(|(i, &count)| {
            let x0 = min + i as f64 * bin_width;
            let x1 = x0 + bin_width;
            Rectangle::new([(x0, 0), (x1, count)], BLUE.filled())
        }))?;
        Ok(())
    }

    fn quartiles(data: &[f64]) -> (f64, f64) {
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = sorted.len();
        let q1_idx = len as f64 * 0.25;
        let q3_idx = len as f64 * 0.75;
        let q1 = sorted[q1_idx.floor() as usize];
        let q3 = sorted[q3_idx.floor() as usize];
        (q1, q3)
    }

    fn mode(data: &[String]) -> String {
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

    fn mean(data: &[f64]) -> f64 {
        let sum: f64 = data.iter().sum();
        sum / (data.len() as f64)
    }

    fn median(data: &[f64]) -> f64 {
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = sorted.len();
        if len.is_multiple_of(2) {
            (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
        } else {
            sorted[len / 2]
        }
    }
}
