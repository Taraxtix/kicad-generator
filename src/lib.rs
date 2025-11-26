#![feature(pattern)]
#![allow(dead_code)] // FIXME: Remove this once the code is more complete

mod parser;
pub mod schematic;

pub const PAGE_WIDTH: usize = 297;
pub const PAGE_HEIGHT: usize = 210;
