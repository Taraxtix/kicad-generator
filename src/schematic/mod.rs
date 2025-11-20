pub mod graphic;
pub mod symbol;
pub mod symbol_library;

use crate::parser;
use crate::schematic::graphic::Graphic;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use symbol::{Symbol, SymbolInstance};
use uuid::Uuid;

#[derive(Debug)]
pub struct KicadSch {
    version: &'static str,
    generator: &'static str,
    generator_version: &'static str,
    uuid: Uuid,
    paper: &'static str,
    lib_symbols: Vec<Symbol>, //Will be written even if empty
    junctions: Vec<Junction>,
    no_connects: Vec<NoConnect>,
    bus_entries: Vec<BusEntry>,
    wires_and_buses: Vec<WireOrBus>,
    images: Vec<Image>,
    polylines: Vec<Graphic>, // Graphic::Polyline
    texts: Vec<Text>,
    labels: Vec<Label>,
    local_labels: Vec<LocalLabel>,
    hierarchical_labels: Vec<HierarchicalLabel>,
    symbols: Vec<SymbolInstance>,
    hierarchical_sheets: Vec<HierarchicalSheet>,
    hierarchical_pins: Vec<HierarchicalPin>,
    page: Page,
}

impl Default for KicadSch {
    fn default() -> Self {
        Self {
            version: "20250114",
            generator: env!("CARGO_PKG_NAME"),
            generator_version: env!("CARGO_PKG_VERSION"),
            uuid: Uuid::new_v4(),
            paper: "A4",
            lib_symbols: vec![],
            junctions: vec![],
            no_connects: vec![],
            bus_entries: vec![],
            wires_and_buses: vec![],
            images: vec![],
            polylines: vec![],
            texts: vec![],
            labels: vec![],
            local_labels: vec![],
            hierarchical_labels: vec![],
            symbols: vec![],
            hierarchical_sheets: vec![],
            hierarchical_pins: vec![],
            page: Page {
                path: "/".to_string(),
                page_number: 1,
            },
        }
    }
}

#[derive(Debug)]
pub struct Junction {}

#[derive(Debug)]
pub struct NoConnect {}

#[derive(Debug)]
pub struct BusEntry {}

#[derive(Debug)]
pub struct WireOrBus {}

#[derive(Debug)]
pub struct Image {}

#[derive(Debug)]
pub struct Text {}

#[derive(Debug)]
pub struct Label {}

#[derive(Debug)]
pub struct LocalLabel {}

#[derive(Debug)]
pub struct HierarchicalLabel {}

#[derive(Debug)]
pub struct HierarchicalSheet {}

#[derive(Debug)]
pub struct HierarchicalPin {}

#[derive(Debug)]
pub struct Page {
    path: String,
    page_number: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    x: f32,
    y: f32,
    rotation: Option<f32>,
}

impl Position {
    pub fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let (position, content) = parser::expect_regex(
            content,
            r#"\(at -?\d+(\.\d+)? -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#,
        )?;
        let position = position.replace("(at ", "").replace(")", "");
        let position = position
            .split(' ')
            .map(|value| f32::from_str(value).expect("Failed to parse float"))
            .collect::<Vec<_>>();
        Ok((
            Self {
                x: position[0],
                y: position[1],
                rotation: position.get(2).copied(),
            },
            content,
        ))
    }
}
