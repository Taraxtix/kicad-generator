pub mod graphic;
pub mod symbol;
pub mod symbol_library;

use std::{fmt::{Display, Formatter},
          str::FromStr};

use serde::{Deserialize, Serialize};
use symbol::{Symbol, SymbolInstance};
use uuid::Uuid;

use crate::{parser, schematic::graphic::Graphic};

#[derive(Debug)]
pub struct KicadSch {
    version: &'static str,
    generator: &'static str,
    generator_version: &'static str,
    uuid: Uuid,
    paper: &'static str,
    lib_symbols: Vec<Symbol>, // Will be written even if empty
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
    pub project_name: String, // Will not be written
}

impl KicadSch {
    pub fn place(&mut self, symbol: &Symbol, position: Position) -> Result<(), String> {
        if !self.lib_symbols.contains(symbol) {
            self.lib_symbols.push(symbol.clone())
        }
        let unit = self.symbols.iter().filter(|s| s.name == symbol.name).count() + 1;
        let symbol_instance = SymbolInstance::from(symbol, position, unit, self)?;
        self.symbols.push(symbol_instance);
        Ok(())
    }
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
            page: Page { path: "/".to_string(), page_number: 1 },
            project_name: "".to_string(),
        }
    }
}

impl Display for KicadSch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(kicad_sch")?;
        f.write_fmt(format_args!("(version {})", self.version))?;
        f.write_fmt(format_args!("(generator {})", self.generator))?;
        f.write_fmt(format_args!("(generator_version {})", self.generator_version))?;
        f.write_fmt(format_args!("(uuid {})", self.uuid))?;
        f.write_fmt(format_args!("(paper {})", self.paper))?;

        f.write_str("(lib_symbols")?;
        for symbol in &self.lib_symbols {
            f.write_fmt(format_args!("\n{}", symbol))?;
        }
        f.write_str(")")?;

        for junction in &self.junctions {
            f.write_fmt(format_args!("\n{}", junction))?;
        }
        for no_connect in &self.no_connects {
            f.write_fmt(format_args!("\n{}", no_connect))?;
        }
        for bus_entry in &self.bus_entries {
            f.write_fmt(format_args!("\n{}", bus_entry))?;
        }
        for wire_or_bus in &self.wires_and_buses {
            f.write_fmt(format_args!("\n{}", wire_or_bus))?;
        }
        for image in &self.images {
            f.write_fmt(format_args!("\n{}", image))?;
        }
        for polyline in &self.polylines {
            f.write_fmt(format_args!("\n{}", polyline))?;
        }
        for text in &self.texts {
            f.write_fmt(format_args!("\n{}", text))?;
        }
        for label in &self.labels {
            f.write_fmt(format_args!("\n{}", label))?;
        }
        for local_label in &self.local_labels {
            f.write_fmt(format_args!("\n{}", local_label))?;
        }
        for hierarchical_label in &self.hierarchical_labels {
            f.write_fmt(format_args!("\n{}", hierarchical_label))?;
        }
        for symbol in &self.symbols {
            f.write_fmt(format_args!("\n{}", symbol))?;
        }
        for hierarchical_sheet in &self.hierarchical_sheets {
            f.write_fmt(format_args!("\n{}", hierarchical_sheet))?;
        }
        for hierarchical_pin in &self.hierarchical_pins {
            f.write_fmt(format_args!("\n{}", hierarchical_pin))?;
        }
        f.write_fmt(format_args!(
            "(sheet_instances\n(path \"{}\"\n(page \"{}\"\n)))",
            self.page.path, self.page.page_number
        ))
    }
}

#[derive(Debug)]
pub struct Junction {}

impl Display for Junction {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct NoConnect {}

impl Display for NoConnect {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct BusEntry {}

impl Display for BusEntry {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct WireOrBus {}

impl Display for WireOrBus {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct Image {}

impl Display for Image {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct Text {}

impl Display for Text {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct Label {}

impl Display for Label {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct LocalLabel {}

impl Display for LocalLabel {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct HierarchicalLabel {}

impl Display for HierarchicalLabel {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct HierarchicalSheet {}

impl Display for HierarchicalSheet {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct HierarchicalPin {}

impl Display for HierarchicalPin {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

#[derive(Debug)]
pub struct Page {
    path: String,
    page_number: usize,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub rotation: Option<f32>,
}

impl Position {
    pub fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let (position, content) =
            parser::expect_regex(content, r#"\(at -?\d+(\.\d+)? -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let position = position.replace("(at ", "").replace(")", "");
        let position = position
            .split(' ')
            .map(|value| f32::from_str(value).expect("Failed to parse float"))
            .collect::<Vec<_>>();
        Ok((Self { x: position[0], y: position[1], rotation: position.get(2).copied() }, content))
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(at {} {} {})", self.x, self.y, self.rotation.unwrap_or(0.0))
    }
}
