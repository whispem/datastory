use crate::analyze::{mean, median, quartiles, mode, pearson_correlation};
// use std::fs::File;
use std::io::Write;
use genpdf::{
    Document,
    elements::{Break, Paragraph},
};

pub fn generate_pdf_report(
    num_columns: &[Vec<f64>],
    _cat_columns: &[Vec<String>],
    headers: &[String],
    path: &str,
    report_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::new(genpdf::fonts::from_files(".", "LiberationSans", None)?);
    doc.set_title("Data Storytelling Report");
    doc.push(Paragraph::new(format!(
        "Narrative report for file: {}",
        path
    )));
    doc.push(Break::new(1));
    doc.push(Paragraph::new("Correlation analysis:"));
    for i in 0..num_columns.len() {
        for j in (i + 1)..num_columns.len() {
            if !num_columns[i].is_empty()
                && !num_columns[j].is_empty()
                && let Some(corr) = crate::analyze::pearson_correlation(&num_columns[i], &num_columns[j])
            {
                doc.push(Paragraph::new(format!(
                    "Pearson correlation between '{}' and '{}': {:.3}",
                    headers[i], headers[j], corr
                )));
            }
        }
    }
    for (i, col) in num_columns.iter().enumerate() {
        if col.is_empty() {
            continue;
        }
        let mean = mean(col);
        let median = median(col);
        let min = col.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = col.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let (q1, q3) = quartiles(col);
        let iqr = q3 - q1;
        let lower = q1 - 1.5 * iqr;
        let upper = q3 + 1.5 * iqr;
        let outliers: Vec<_> = col
            .iter()
            .cloned()
            .filter(|v| *v < lower || *v > upper)
            .collect();
        doc.push(Break::new(1));
        doc.push(Paragraph::new(format!("Column '{}':", headers[i])));
        doc.push(Paragraph::new("Type: numeric".to_string()));
        doc.push(Paragraph::new(format!("Min: {:.2}, Max: {:.2}, Mean: {:.2}, Median: {:.2}, Q1: {:.2}, Q3: {:.2}, IQR: {:.2}, Outliers detected: {}", min, max, mean, median, q1, q3, iqr, outliers.len())));
        if !outliers.is_empty() {
            doc.push(Paragraph::new(format!("Extreme values: {:?}", outliers)));
        }
        let suggestions = crate::analyze::cleaning_suggestions(col);
        doc.push(Paragraph::new("Cleaning suggestions:"));
        for s in suggestions {
            doc.push(Paragraph::new(format!("- {}", s)));
        }
    }
    // Optionally: save the PDF to disk
    doc.render_to_file(report_name)?;
    Ok(())
}

pub fn generate_html_report(
    num_columns: &[Vec<f64>],
    cat_columns: &[Vec<String>],
    headers: &[String],
    path: &str,
    report_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    let mut html = String::new();
    html.push_str(&format!("<html><head><title>Data Storytelling Report</title></head><body>\n<h1>Data Storytelling Report</h1><h2>File: {}</h2>", path));
    html.push_str("<h2>Correlation analysis</h2>");
    for i in 0..num_columns.len() {
        for j in (i + 1)..num_columns.len() {
            if !num_columns[i].is_empty()
                && !num_columns[j].is_empty()
                && let Some(corr) = pearson_correlation(&num_columns[i], &num_columns[j])
            {
                html.push_str(&format!(
                    "<li>Pearson correlation between '{}' and '{}': {:.3}</li>",
                    headers[i], headers[j], corr
                ));
            }
        }
    }
    // Numeric columns
    for (i, col) in num_columns.iter().enumerate() {
        if col.is_empty() {
            continue;
        }
        let mean = mean(col);
        let median = median(col);
        let min = col.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = col.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let (q1, q3) = quartiles(col);
        let iqr = q3 - q1;
        let lower = q1 - 1.5 * iqr;
        let upper = q3 + 1.5 * iqr;
        let outliers: Vec<_> = col.iter().cloned().filter(|v| *v < lower || *v > upper).collect();
        html.push_str(&format!(
            "<h2>Column '{}'</h2><ul><li>Type: numeric</li>", &headers[i]
        ));
        html.push_str(&format!("<li>Min: {:.2}</li>", min));
        html.push_str(&format!("<li>Max: {:.2}</li>", max));
        html.push_str(&format!("<li>Mean: {:.2}</li>", mean));
        html.push_str(&format!("<li>Median: {:.2}</li>", median));
        html.push_str(&format!("<li>Q1: {:.2}, Q3: {:.2}, IQR: {:.2}</li>", q1, q3, iqr));
        html.push_str(&format!("<li>Outliers detected: {}</li>", outliers.len()));
        if !outliers.is_empty() {
            html.push_str(&format!("<li>Extreme values: {:?}</li>", outliers));
        }
        html.push_str("</ul>");
        html.push_str(&format!("<p>Narrative: Column '{}' has a mean value of {:.2}, ranging from {:.2} to {:.2}. The median is {:.2}. {} </p>",
            &headers[i], mean, min, max, median,
            if outliers.is_empty() {
                "No extreme values detected."
            } else {
                "Extreme values were detected, which may indicate anomalies or data entry errors."
            }
        ));
        let img_name = format!("assets/hist_{}.png", i);
        let boxplot_img = format!("assets/boxplot_{}.png", i);
        if let Err(e) = save_histogram(&img_name, col, &headers[i]) {
            eprintln!("Error generating chart: {}", e);
        }
        html.push_str(&format!("<img src='{}' alt='Histogram {}'/><br/>", img_name, &headers[i]));
        if let Err(e) = save_boxplot(&boxplot_img, col, &headers[i]) {
            eprintln!("Error generating boxplot: {}", e);
        }
        html.push_str(&format!("<img src='{}' alt='Boxplot {}'/><br/>", boxplot_img, &headers[i]));
    }
    // Categorical columns
    for (i, col) in cat_columns.iter().enumerate() {
        if col.is_empty() {
            continue;
        }
        let col_type = crate::analyze::detect_column_type(col);
        let unique: std::collections::HashSet<_> = col.iter().cloned().collect();
        let mode_val = mode(col);
        let freq = col.iter().filter(|v| **v == mode_val).count();
        html.push_str(&format!(
            "<h2>Column '{}'</h2><ul><li>Type: {}</li>", &headers[i], col_type
        ));
        html.push_str(&format!("<li>Unique values: {}</li>", unique.len()));
        html.push_str(&format!("<li>Mode: '{}' ({} occurrences)</li>", mode_val, freq));
        html.push_str("</ul>");
        html.push_str(&format!("<p>Narrative: Column '{}' contains {} unique values. The most frequent value is '{}' ({} times).</p>", &headers[i], unique.len(), mode_val, freq));
        let bar_img = format!("assets/bar_{}.png", i);
        if let Err(e) = save_bar_chart(&bar_img, col, &headers[i]) {
            eprintln!("Error generating bar chart: {}", e);
        }
        html.push_str(&format!("<img src='{}' alt='Bar Chart {}'/><br/>", bar_img, &headers[i]));
    }
    html.push_str("</body></html>");
    let mut file = File::create(report_name)?;

    file.write_all(html.as_bytes())?;

    Ok(())
}

pub fn save_histogram(
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
pub fn save_boxplot(
    filename: &str,
    data: &[f64],
    colname: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;
    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let (q1, q3) = crate::analyze::quartiles(data);
    let median = crate::analyze::median(data);
    let iqr = q3 - q1;
    let lower_whisker = data
        .iter()
        .cloned()
        .filter(|v| *v >= (q1 - 1.5 * iqr))
        .fold(q1, f64::min);
    let upper_whisker = data
        .iter()
        .cloned()
        .filter(|v| *v <= (q3 + 1.5 * iqr))
        .fold(q3, f64::max);
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Boxplot - {}", colname), ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..1, min..max)?;
    chart.configure_mesh().disable_x_mesh().draw()?;
    chart.draw_series(std::iter::once(Rectangle::new(
        [(0, q1), (1, q3)],
        RED.filled(),
    )))?;
    chart.draw_series(std::iter::once(Rectangle::new(
        [(0, median), (1, median)],
        BLACK,
    )))?;
    chart.draw_series(std::iter::once(Rectangle::new(
        [(0, lower_whisker), (1, lower_whisker)],
        BLUE,
    )))?;
    chart.draw_series(std::iter::once(Rectangle::new(
        [(0, upper_whisker), (1, upper_whisker)],
        BLUE,
    )))?;
    Ok(())
}
pub fn save_bar_chart(
    filename: &str,
    data: &[String],
    colname: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;
    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    // Compute frequencies
    let mut freq_map = std::collections::HashMap::new();
    for val in data {
        *freq_map.entry(val).or_insert(0) += 1;
    }
    let max_count = *freq_map.values().max().unwrap_or(&0);
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Bar Chart - {}", colname), ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..freq_map.len(), 0..max_count)?;
    chart.configure_mesh().draw()?;
    for (i, (_val, count)) in freq_map.iter().enumerate() {
        chart.draw_series(std::iter::once(Rectangle::new(
            [(i, 0), (i + 1, *count)], BLUE.filled(),
        )))?;
    }
    Ok(())
}
