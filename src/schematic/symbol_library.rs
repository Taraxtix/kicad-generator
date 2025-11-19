use crate::parser;
use crate::schematic::symbol::Symbol;
use log::{debug, info};
use std::fs::read_to_string;
use std::path::Path;

pub struct SymbolLibrary {
    version: String,
    generator: String,
    generator_version: String,
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
        let content = parser::expect_str(content, "(kicad_symbol_lib")?;
        let (version, content) = parser::expect_regex(content, r"\(version \d+\)")?;
        let version = version[9..version.len() - 1].to_string();
        debug!("Version: {version}");
        let (generator, content) = parser::expect_regex(content, r#"\(generator "[^"]+"\)"#)?;
        let generator = generator[12..generator.len() - 2].to_string();
        debug!("Generator: {generator}");
        let (generator_version, mut content) =
            parser::expect_regex(content, r#"\(generator_version "[^"]+"\)"#)?;
        let generator_version = generator_version[20..generator_version.len() - 2].to_string();
        debug!("Generator Version: {generator_version}\n");

        let mut symbols = vec![];
        while (content.starts_with("(symbol")) {
            let (symbol, rest) = Symbol::extract_from(content)?;
            symbols.push(symbol);
            content = rest;
        }

        info!("Found {} symbols which names are:", symbols.len());
        for symbol in symbols.iter() {
            info!("\t{}", symbol.name);
        }
        Ok(Self {
            version,
            generator,
            generator_version,
            symbols,
        })
    }
}
