use std::fmt::{Display, Formatter};

use log::{debug, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{parser,
            schematic::{graphic::{Graphic, TextEffect},
                        KicadSch,
                        Position}};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    exclude_from_sim: bool,
    pin_names: (Option<f32>, bool), //[(pin_names [offset OFFSET] hide)]
    pin_numbers: bool,              /* (pin_numbers hide) => true if it exists (hidden), false
                                     * otherwise */
    in_bom: bool,
    on_board: bool,
    properties: Vec<Property>,
    graphics: Vec<Graphic>,
    pins: Vec<Graphic>, // Graphic::Pin
    units: Vec<Symbol>,
    unit_name: Option<String>,
}

impl Symbol {
    pub fn extract_from<'a>(
        content: &'a str,
        lib_name: &'a String,
    ) -> Result<(Self, &'a str), String> {
        let content = content.trim();
        let (name, mut content) = parser::expect_regex(content, r#"\(symbol "[^"]+""#)?;
        let name = name[9..name.len() - 1].to_string();
        debug!("Symbol Name: {name}");
        let mut it = Self {
            name: format!("{}:{}", lib_name, name),
            exclude_from_sim: true,
            pin_names: (None, false),
            pin_numbers: false,
            in_bom: false,
            on_board: false,
            properties: vec![],
            graphics: vec![],
            pins: vec![],
            units: vec![],
            unit_name: None,
        };
        loop {
            if content.starts_with(")") {
                break;
            } else if content.starts_with("(in_bom") {
                let (in_bom, left) = parser::expect_regex(content, r#"\(in_bom (yes|no)\)"#)?;
                let in_bom = &in_bom[8..in_bom.len() - 1];
                debug!("In BOM: {in_bom}");
                it.in_bom = in_bom == "yes";
                content = left;
            } else if content.starts_with("(exclude_from_sim no)") {
                content = parser::expect_str(content, "(exclude_from_sim no)")?;
                it.exclude_from_sim = false;
            } else if content.starts_with("(on_board") {
                let (on_board, left) = parser::expect_regex(content, r#"\(on_board (yes|no)\)"#)?;
                let on_board = &on_board[10..on_board.len() - 1];
                debug!("On board: {on_board}");
                it.on_board = on_board == "yes";
                content = left;
            } else if content.starts_with("(property") {
                let (property, left) = Property::extract_from(content)?;
                it.properties.push(property);
                content = left;
            } else if content.starts_with("(symbol") {
                let (unit, left) = Self::extract_from(content, lib_name)?;
                it.units.push(unit);
                content = left;
            } else if content.starts_with("(rectangle") {
                let (rectangle, left) = Graphic::extract_rectangle_from(content)?;
                it.graphics.push(rectangle);
                content = left;
            } else if content.starts_with("(circle") {
                let (circle, left) = Graphic::extract_circle_from(content)?;
                it.graphics.push(circle);
                content = left;
            } else if content.starts_with("(arc") {
                let (arc, left) = Graphic::extract_arc_from(content)?;
                it.graphics.push(arc);
                content = left;
            } else if content.starts_with("(pin_names") {
                content = parser::expect_str(content, "(pin_names")?;
                let offset_opt;
                if content.starts_with("(offset") {
                    let offset;
                    (offset, content) =
                        parser::expect_regex(content, r#"\(offset -?\d+(\.\d+)?\)"#)?;
                    let offset = offset.replace("(offset ", "").replace(")", "");
                    let offset = offset
                        .parse::<f32>()
                        .map_err(|e| format!("Trying to convert `{offset}` to f32: {e}"))?;
                    offset_opt = Some(offset);
                } else {
                    offset_opt = None;
                }
                if content.starts_with("hide)") {
                    content = parser::expect_str(content, "hide)")?;
                    it.pin_names = (offset_opt, true);
                } else {
                    content = parser::expect_str(content, ")")?;
                    it.pin_names = (offset_opt, false);
                }
            } else if content.starts_with("(pin_numbers") {
                content = parser::expect_str(content, "(pin_numbers hide)")?;
                it.pin_numbers = true;
            } else if content.starts_with("(pin") {
                let (pin, left) = Graphic::extract_pin_from(content)?;
                it.graphics.push(pin);
                content = left;
            } else if content.starts_with("(polyline") {
                let (polyline, left) = Graphic::extract_polyline_from(content)?;
                it.graphics.push(polyline);
                content = left;
            } else {
                let (skipped, left) = parser::expect_regex(content, r#"\([^\)]*\)"#)?;
                warn!("Skipped: {}", skipped);
                content = left;
            }
        }
        let content = parser::expect_str(content, ")")?;
        Ok((it, content))
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("(symbol")?;
        f.write_fmt(format_args!("\n\"{}\"", self.name))?;
        // f.write_fmt(format_args!(
        //     "(exclude_from_sim {})",
        //     if self.exclude_from_sim { "no" } else { "yes" }
        // ))?;
        // if self.pin_numbers {
        //     f.write_str("(pin_numbers hide)")?;
        // }
        // if let Some(offset) = self.pin_names.0 {
        //     f.write_fmt(format_args!(
        //         "(pin_names (offset {}) {})",
        //         offset,
        //         if self.pin_names.1 { "hide" } else { "" }
        //     ))?;
        // }
        // f.write_fmt(format_args!("(in_bom {})", if self.in_bom { "yes" } else { "no" }))?;
        // f.write_fmt(format_args!("(on_board {})", if self.on_board { "yes" } else { "no" }))?;
        // for property in &self.properties {
        //     f.write_fmt(format_args!("\n{}", property))?;
        // }
        for graphic in &self.graphics {
            f.write_fmt(format_args!("\n{}", graphic))?;
        }
        for pin in &self.pins {
            f.write_fmt(format_args!("\n{}", pin))?;
        }
        for unit in &self.units {
            let mut unit = unit.clone();
            unit.name = unit.name.split_once(':').unwrap().1.to_string();
            f.write_fmt(format_args!("\n{}", unit))?;
        }
        if let Some(unit_name) = &self.unit_name {
            f.write_fmt(format_args!("(unit_name {})", unit_name))?;
        }
        f.write_str(")")
    }
}

#[derive(Debug)]
pub struct SymbolInstance {
    pub name: String,
    position: Position,
    unit: usize,
    in_bom: bool,
    on_board: bool,
    uuid: Uuid,
    properties: Vec<Property>,
    pins: Vec<Pin>,
    instance: Instance, // FIXME: Should be Vec<Instance> (Also see fixme of Instance type)
}

impl Display for SymbolInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(symbol\n(lib_id \n\"{name}\")\n{position}\n(unit {unit})\n(in_bom \
             {in_bom})\n(on_board {on_board})\n(uuid \"{uuid}\")",
            name = self.name,
            position = self.position,
            unit = self.unit,
            in_bom = if self.in_bom { "yes" } else { "no" },
            on_board = if self.on_board { "yes" } else { "no" },
            uuid = self.uuid
        ))?;
        for property in &self.properties {
            let mut property = property.clone();
            property.position = Position {
                x: self.position.x + property.position.x,
                y: self.position.y + property.position.y,
                rotation: Some(
                    self.position.rotation.unwrap_or(0.0)
                        + property.position.rotation.unwrap_or(0.0),
                ),
            };
            f.write_fmt(format_args!("\n{}", property))?;
        }
        for pin in &self.pins {
            f.write_fmt(format_args!(
                "\n(pin \"{name}\" (uuid {uuid}))",
                name = pin.name,
                uuid = pin.uuid
            ))?;
        }
        f.write_fmt(format_args!(
            "\n(instances\n(project \"{project_name}\" (path \"{path}\" (reference \
             \"{reference}\") (unit {unit}))))))",
            project_name = self.instance.project_name,
            path = self.instance.path.path,
            reference = self.instance.path.reference,
            unit = self.instance.path.unit
        ))
    }
}

impl SymbolInstance {
    pub fn from(
        symbol: &Symbol,
        position: Position,
        unit: usize,
        sheet: &KicadSch,
    ) -> Result<Self, String> {
        let base_reference = symbol
            .properties
            .iter()
            .find(|p| p.name == "Reference")
            .ok_or("Symbol doesn't contains a Reference property.")?
            .value
            .clone();

        let pins = symbol.pins.iter().map(Pin::from).collect();

        Ok(Self {
            name: symbol.name.clone(),
            position,
            unit,
            in_bom: true,
            on_board: true,
            uuid: Uuid::new_v4(),
            properties: symbol.properties.clone(),
            pins,
            instance: Instance {
                project_name: sheet.project_name.clone(),
                path: InstancePath {
                    path: format!("/{}", sheet.uuid),
                    reference: format!("{}{}", base_reference, unit),
                    unit,
                },
            },
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
struct Property {
    name: String,
    value: String,
    position: Position,
    do_not_autoplace: bool,
    text_effect: TextEffect,
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(property \"{name}\" \"{value}\" {position}",
            name = self.name,
            value = self.value,
            position = self.position,
        ))?;
        if self.do_not_autoplace {
            f.write_str(" (do_not_autoplace)")?;
        }
        f.write_fmt(format_args!(" {})", self.text_effect))
    }
}

impl Property {
    fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let (name_and_value, content) =
            parser::expect_regex(content, r#"\(property "[^"]+" "[^"]*""#)?;
        let name_and_value = name_and_value.replace("(property ", "").replace("\"", "");
        let (name, value) = name_and_value.split_once(' ').unwrap();
        let (name, value) = (name.to_string(), value.to_string());
        let (position, content) = Position::extract_from(content)?;

        let do_not_autoplace = content.starts_with("(do_not_autoplace)");

        let content = if do_not_autoplace {
            parser::expect_str(content, "(do_not_autoplace)")?
        } else {
            content
        };

        let (text_effect, content) = TextEffect::extract_from(content)?;
        let content = parser::expect_str(content, ")")?;
        Ok((Self { name, value, do_not_autoplace, position, text_effect }, content))
    }
}

#[derive(Debug)]
struct Pin {
    name: String, // Usually the pin number
    uuid: Uuid,
}

impl Pin {
    pub fn from(pin: &Graphic) -> Self {
        let Graphic::Pin { number, .. } = pin else { unreachable!("Should always be a pin") };
        Self { name: format!("{number}"), uuid: Uuid::new_v4() }
    }
}

#[derive(Debug)]
struct Instance {
    project_name: String,
    path: InstancePath, // FIXME: Should be Vec<InstancePath>
}

#[derive(Debug)]
pub struct InstancePath {
    path:      String, // Usually "/{project_uuid}"
    reference: String, // Example "U2"
    unit:      usize,
}

// #[derive(Debug)]
// pub struct PinOffset {
//     offset: f32,
//     hide: bool,
// }
