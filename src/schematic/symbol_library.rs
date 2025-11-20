use crate::parser;
use crate::schematic::symbol::Symbol;
use log::{debug, info};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct SymbolLibrary {
    pub name: String,
    version: String,
    generator: String,
    generator_version: String,
    symbols: Vec<Symbol>,
}

impl SymbolLibrary {
    pub fn get_statics() -> Vec<Self> {
        let path = Path::new("static/included_libs");
        let mut libraries = vec![];
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let str = read_to_string(&path).unwrap();
            libraries.push(serde_json::from_str(&str).unwrap());
        }
        libraries
    }
    
    pub fn add_dir_to(libraries: &mut Vec<Self>, path: impl AsRef<Path>) -> Result<(), String> {
        let libraries_name = libraries
            .iter()
            .map(|lib| lib.name.clone())
            .collect::<Vec<_>>();

        let paths = Self::get_all_lib_files(path)?;
        let paths = paths
            .iter()
            .filter(|path| {
                !libraries_name.contains(&path.file_stem().unwrap().to_string_lossy().to_string())
            })
            .collect::<Vec<_>>();

        libraries.extend(
            paths
                .par_iter()
                .map(|path| {
                    info!("Loading symbol library from {}", path.to_string_lossy());
                    Self::from_path(path)
                })
                .collect::<Result<Vec<_>, _>>()?,
        );

        Ok(())
    }

    pub fn all_from_dir(path: impl AsRef<Path>) -> Result<Vec<Self>, String> {
        let paths = Self::get_all_lib_files(path)?;
        paths
            .par_iter()
            .map(|path| {
                info!("Loading symbol library from {}", path.to_string_lossy());
                Self::from_path(path)
            })
            .collect()
    }

    fn get_all_lib_files(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, String> {
        let path = path.as_ref();
        if !path.is_dir() {
            Err(format!("{} is not a directory", path.to_string_lossy()))
        } else {
            let mut paths = vec![];

            for entry in path.read_dir().map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();

                if path.is_file() && path.extension().unwrap_or_default() == "kicad_sym" {
                    paths.push(path);
                } else if path.is_dir() {
                    paths.extend(Self::get_all_lib_files(path)?);
                } else {
                    return Err(format!(
                        "{} is neither a file nor a directory",
                        path.to_string_lossy()
                    ));
                }
            }
            Ok(paths)
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, String> {
        let src = read_to_string(&path).map_err(|e| e.to_string())?;
        let path = path.as_ref();

        Self::from_string(
            src,
            path.file_stem()
                .expect("Should have a filename if we read it")
                .to_string_lossy()
                .to_string(),
        )
    }

    pub fn from_string(content: impl Into<String>, name: String) -> Result<Self, String> {
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
        while content.starts_with("(symbol") {
            let (symbol, rest) = Symbol::extract_from(content)?;
            symbols.push(symbol);
            content = rest;
        }

        info!(
            "Found {} symbols for library {name}. symbols names are:",
            symbols.len()
        );
        for symbol in symbols.iter() {
            info!("\t{}", symbol.name);
        }
        Ok(Self {
            name,
            version,
            generator,
            generator_version,
            symbols,
        })
    }
}
