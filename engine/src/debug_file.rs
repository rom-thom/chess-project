

use std::{fs::OpenOptions, io::Write, path::Path};

/// Appends `line` to `path`. Creates the file if it doesn't exist.
/// Best-effort: returns Err if opening/writing fails.
pub fn append_line<P: AsRef<Path>>(path: P, line: &str) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    f.write_all(line.as_bytes())?;
    Ok(())
}


pub fn log_dbg<P: AsRef<Path>, T: std::fmt::Debug>(
    path: P,
    label: &str,
    value: Option<&T>,
    file: &str,
    line: u32,
) -> std::io::Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    match value {
        Some(val) => writeln!(f, "[{}:{}] {} = {:#?}", file, line, label, val)?,
        None => writeln!(f, "[{}:{}] {}", file, line, label)?

    }
    Ok(())
}