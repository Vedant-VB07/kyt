use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::HashSet;
use anyhow::Result;

pub fn write_to_file(filename: &str, data: &HashSet<String>) -> Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for line in data {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}