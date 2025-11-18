use crate::schematic::symbol::Symbol;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

pub struct SymbolLibrary {
    version: String,
    generator: String,
    generate_version: String,
    symbols: Vec<Symbol>,
}

impl SymbolLibrary {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, String> {
        let src = read_to_string(path).map_err(|e| e.to_string())?;
        Self::from_string(src)
    }

    pub fn from_string(content: impl Into<String>) -> Result<Self, String> {
        let content = content.into();
        let content = content.trim();
        let content = Self::expect_str(content, "(kicad_symbol_lib")?;
        let (version, content) = Self::expect_regex(content, r"\(version \d+\)")?;
        let version = version[9..version.len() - 1].to_string();
        println!("Version: {}", version);
        todo!("{content}")
        // Ok(Self{
        //     version,
        //     generator,
        //     generate_version,
        //     symbols: vec![],
        // })
    }

    fn expect_str<'a>(content: &'a str, pattern: &'static str) -> Result<&'a str, String> {
        if let Some(stripped) = content.strip_prefix(pattern) {
            println!("Got: {}", &content[..pattern.len()]);
            Ok(stripped.trim())
        } else {
            Err(format!(
                "Expected {pattern}, but got {}",
                content.split_at(pattern.len() + 1).0
            ))
        }
    }

    fn expect_regex<'a>(
        content: &'a str,
        pattern: &'static str,
    ) -> Result<(&'a str, &'a str), String> {
        let regex = Regex::new(pattern).map_err(|e| e.to_string())?;
        if let Some(found) = regex.find(content) {
            if found.start() != 0 {
                Err(format!("Expected {regex}, but got {}", content))
            } else {
                Ok((&content[..found.end()], &content[found.end()..]))
            }
        } else {
            Err(format!("Expected {regex}, but got {}", content))
        }
    }
}
