use parquet::record::RowAccessor;
// File format readers: CSV, Parquet, JSON

pub fn read_csv(path: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
    let mut data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        data.push(record.iter().map(|s| s.to_string()).collect());
    }
    Ok(data)
}

pub fn read_parquet(path: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    use parquet::file::reader::{FileReader, SerializedFileReader};
    use std::fs::File;
    let file = File::open(path)?;
    let reader = SerializedFileReader::new(file)?;
    let mut data = Vec::new();
    for record in reader.get_row_iter(None)? {
        let mut row = Vec::new();
        for i in 0..record.len() {
            row.push(record.get_string(i).map_or("", |v| v).to_string());
        }
        data.push(row);
    }
    Ok(data)
}

pub fn read_json(path: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    use std::fs;
    let content = fs::read_to_string(path)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;
    let mut data = Vec::new();
    if let Some(arr) = json.as_array() {
        for obj in arr {
            let mut row = Vec::new();
            if let Some(map) = obj.as_object() {
                for (_k, v) in map.iter() {
                    row.push(v.to_string());
                }
            }
            data.push(row);
        }
    }
    Ok(data)
}
