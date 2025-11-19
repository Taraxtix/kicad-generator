use regex::Regex;

pub fn expect_str<'a>(content: &'a str, pattern: &'static str) -> Result<&'a str, String> {
    if let Some(stripped) = content.strip_prefix(pattern) {
        Ok(stripped.trim())
    } else {
        Err(format!(
            "Expected {pattern}, but got {}",
            content.split_at(pattern.len() + 1).0
        ))
    }
}

pub fn expect_regex<'a>(
    content: &'a str,
    pattern: &'static str,
) -> Result<(&'a str, &'a str), String> {
    let regex = Regex::new(pattern).map_err(|e| e.to_string())?;
    if let Some(found) = regex.find(content) {
        if found.start() != 0 {
            Err(format!("Expected {regex}, but got {}", content))
        } else {
            Ok((&content[..found.end()], content[found.end()..].trim()))
        }
    } else {
        Err(format!("Expected {regex}, but got {}", content))
    }
}
