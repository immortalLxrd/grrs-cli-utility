use std::io::Write;
use anyhow::Result;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> Result<()> {
    if content.contains(pattern) {
        writeln!(writer, "{}", content)?;
    }
    Ok(())
}
