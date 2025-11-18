use crate::schematic::graphic::{Graphic, TextEffect};
use uuid::Uuid;

#[derive(Debug)]
pub struct Symbol {
    library_id: String,
    in_bom: bool,
    on_board: bool,
    properties: Vec<Property>,
    graphics: Vec<Graphic>,
    pins: Vec<Graphic>, //Graphic::Pin
    units: Vec<Symbol>,
    unit_name: Option<String>,
}

#[derive(Debug)]
pub struct SymbolInstance {
    library_id: String,
    position: (f32, f32, f32),
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
    id: usize,                         // (id N)
    position: (f32, f32, Option<f32>), // (at x y [rotation])
    text_effect: TextEffect,
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
