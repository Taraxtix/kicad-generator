use crate::parser;
use crate::schematic::graphic::{Graphic, TextEffect};
use crate::schematic::Position;
use log::{debug, warn};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    in_bom: bool,
    on_board: bool,
    properties: Vec<Property>,
    graphics: Vec<Graphic>,
    pins: Vec<Graphic>, //Graphic::Pin
    units: Vec<Symbol>,
    unit_name: Option<String>,
}

impl Symbol {
    pub fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let content = content.trim();
        let (name, mut content) = parser::expect_regex(content, r#"\(symbol "[^"]+""#)?;
        let name = name[9..name.len() - 1].to_string();
        debug!("Symbol Name: {name}");
        let mut it = Self {
            name,
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
                content = left.trim();
            } else if content.starts_with("(on_board") {
                let (on_board, left) = parser::expect_regex(content, r#"\(on_board (yes|no)\)"#)?;
                let on_board = &on_board[10..on_board.len() - 1];
                debug!("On board: {on_board}");
                it.on_board = on_board == "yes";
                content = left.trim();
            } else if content.starts_with("(property") {
                let (property, left) = Property::extract_from(content)?;
                it.properties.push(property);
                content = left;
            } else if content.starts_with("(symbol") {
                let (unit, left) = Self::extract_from(content)?;
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
            } else if content.starts_with("(pin") {
                let (circle, left) = Graphic::extract_pin_from(content)?;
                it.graphics.push(circle);
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

#[derive(Debug)]
pub struct SymbolInstance {
    name: String,
    position: Position,
    unit: usize,
    in_bom: bool,
    on_board: bool,
    uuid: Uuid,
    properties: Vec<Property>,
    pins: Vec<Pin>,
    instance: Instance, //FIXME: Should be Vec<Instance> (Also see fixme of Instance type)
}

#[derive(Debug)]
struct Property {
    name: String,
    value: String,
    position: Position,
    text_effect: TextEffect,
}

impl Property {
    fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let (name_and_value, content) =
            parser::expect_regex(content, r#"\(property "[^"]+" "[^"]+""#)?;
        let name_and_value = name_and_value.replace("(property ", "").replace("\"", "");
        let (name, value) = name_and_value.split_once(' ').unwrap();
        let (name, value) = (name.to_string(), value.to_string());
        debug!("Property \"{name}\" = {value}");
        let (position, content) = Position::extract_from(content)?;
        debug!("\tPosition: {position:?}");
        let (text_effect, content) = TextEffect::extract_from(content)?;
        debug!("\tText Effect: {:?}", text_effect);
        let content = parser::expect_str(content, ")")?;
        Ok((
            Self {
                name,
                value,
                position,
                text_effect,
            },
            content,
        ))
    }
}

#[derive(Debug)]
struct Pin {
    name: String, //Usually the pin number
    uuid: Uuid,
}

#[derive(Debug)]
struct Instance {
    name: String,
    project_name: String,
    path: InstancePath, //FIXME: Should be Vec<InstancePath>
}

#[derive(Debug)]
pub struct InstancePath {
    path: String,      //Usually "/{project_uuid}"
    reference: String, // Example "U2"
    unit: usize,
}

// #[derive(Debug)]
// pub struct PinOffset {
//     offset: f32,
//     hide: bool,
// }
