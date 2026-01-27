# datastory

This project is a Rust CLI and Python API tool for automated data storytelling. It ingests CSV, Parquet, or JSON files, analyzes the data, and generates narrative reports with visualizations (histograms, etc.). The architecture is modular and extensible for future analysis modules or export formats.

## Features
- Ingest CSV, Parquet, and JSON files
- Automatic type detection for columns
- Descriptive statistics (mean, median, quartiles, mode)
- Outlier detection (IQR method)
- Correlation analysis (Pearson)
- Data cleaning suggestions
- Narrative report generation (HTML, Markdown, PDF)
- Visualizations: histogram, boxplot, bar chart, correlation heatmap
- Python API (via pyo3)
- Modular codebase for easy extension

## Supported Formats
- Input: CSV, Parquet, JSON
- Output: HTML, Markdown, PDF

## Installation
Clone the repository and build with Cargo:
```
git clone https://github.com/whispem/datastory.git
cd datastory
cargo build --release
```

## Usage

### CLI
Run the tool on a data file (CSV, Parquet, or JSON):
```
cargo run -- path/to/data.csv [output_report.html]
```
Options:
- `output_report.html` (optional): specify the output report filename
- More options coming soon (detail level, report format)

The report will be generated in the project folder.

### Python API
Build the Python extension with maturin or setuptools-rust, then use in Python:
```python
from datastory import analyze_csv_py
analyze_csv_py('path/to/data.csv')
# The HTML report will be generated in the project folder
```

#### Example Jupyter Notebook
```python
import pandas as pd
from datastory import analyze_csv_py

# Load and preview your data
df = pd.read_csv('mydata.csv')
print(df.head())

# Generate the report
analyze_csv_py('mydata.csv')
# Open 'rapport.html' in your browser
```

## Example: CLI Usage

Analyze a CSV file and generate a full HTML report with all visualizations:
```
cargo run -- data/example.csv report.html
```
This will produce:
- Descriptive statistics
- Outlier detection
- Automatic type detection
- Correlation analysis
- Cleaning suggestions
- Visualizations: histogram, boxplot, correlation heatmap, bar chart
- Narrative summary

## Example: Python API Usage

```python
from datastory import analyze_csv_py
analyze_csv_py('data/example.csv')
```
This will generate the same report as the CLI, including all visualizations.

## Output Visualizations
- Histogram for each numeric column
- Boxplot for each numeric column
- Bar chart for each categorical column
- Correlation heatmap for numeric columns

All images are saved in the working directory and embedded in the report.

## Modularity & Extensibility
The codebase is fully modular:
- Each analysis, format, and report logic is in its own file (src/analyze.rs, src/formats.rs, src/report.rs)
- Add new analysis modules or export formats by creating new files and updating main.rs
- Visualizations are easily extendable (add new chart types in src/report.rs)
- Python API is exposed via pyo3 for seamless integration

## Real Dataset Examples
You can test the tool with real-world datasets:
- [Iris dataset](https://archive.ics.uci.edu/ml/machine-learning-databases/iris/iris.data)
- [Titanic dataset](https://raw.githubusercontent.com/datasciencedojo/datasets/master/titanic.csv)
- [NYC Taxi Trips](https://www1.nyc.gov/site/tlc/about/tlc-trip-record-data.page)
- [Wine Quality](https://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality/winequality-red.csv)

Example:
```
cargo run -- data/iris.csv report.html
```

## Continuous Integration
This project uses GitHub Actions for CI:
- Automatic build, test, formatting, and linting on every push or PR
- See .github/workflows/ci.yml for details

## Roadmap
- Add more visualizations (boxplots, bar charts)
- Add correlation analysis
- Add data cleaning suggestions
- Improve CLI options and documentation
- Add more export formats (PDF, Markdown)

## Project Structure
- `src/main.rs`: CLI entry point and Python API
- `src/formats.rs`: File format readers (CSV, Parquet, JSON)
- `src/analyze.rs`: Analysis and statistics functions
- `src/report.rs`: Report generation and visualizations

## Tests
Run all unit tests:
```
cargo test
```

---
This README will be updated as the project evolves.
