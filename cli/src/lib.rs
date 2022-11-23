use std::io::Write;

use anyhow::{Ok, Result};

pub fn find_matches(content: &str, pattern: &str, writer: &mut impl Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_match() -> Result<()> {
        let mut result = Vec::new();
        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
        assert_eq!(result, b"lorem ipsum\n");
        Ok(())
    }
}
