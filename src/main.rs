use std::{fs::OpenOptions, io::Write, path::Path, sync::OnceLock};

use clap::Parser;
use kicad_generator::{schematic::{symbol_library::SymbolLibraries, KicadSch, Position},
                      PAGE_HEIGHT,
                      PAGE_WIDTH};
use lazy_static::lazy_static;

lazy_static! {
    static ref KICAD_3RD_PARTY_PATH: OnceLock<String> = OnceLock::new();
    static ref KICAD_SYMBOL_LIBRARIES_PATH: OnceLock<String> = OnceLock::new();
}

// A program to parse, modify generate KiCad schematics
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    project_directory: String,

    #[clap(short = 'o', long, default_value = "output.sch")]
    sheet_output: Option<String>,

    /// Allows overriding the default path to the 3rd party libraries
    #[clap(long, default_value = "$HOME/.local/share/kicad/9.0/3rdparty/symbols")]
    kicad_3rd_party_path: String,

    /// Allows overriding the default path to kicad9's symbol libraries
    #[clap(long, default_value = "/usr/share/kicad/symbols")]
    kicad_symbol_path: String,

    #[clap(short, long, default_value = "false")]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    env_logger::init();

    let path = args.kicad_3rd_party_path.replace("$HOME", env!("HOME"));
    KICAD_3RD_PARTY_PATH.set(path).expect("Should be the first time setting KICAD_3RD_PARTY_PATH");
    KICAD_SYMBOL_LIBRARIES_PATH
        .set(args.kicad_symbol_path)
        .expect("Should be the first time setting KICAD_SYMBOL_LIBRARIES_PATH");

    let project_path = Path::new(&args.project_directory).canonicalize().unwrap();
    let project_name = project_path.to_string_lossy().to_string();
    let project_name = project_name.rsplit_once("/").unwrap().1;

    let mut schematic = KicadSch::default();

    schematic.project_name = project_name.to_string();

    let mut symbol_libraries = SymbolLibraries::get_statics();
    // let mut symbol_libraries =
    //     SymbolLibraries::all_from_dir(KICAD_3RD_PARTY_PATH.get().unwrap()).unwrap();
    symbol_libraries.add_dir(KICAD_3RD_PARTY_PATH.get().unwrap()).unwrap();

    for lib in symbol_libraries.iter() {
        let path = format!("static/included_libs/{}", lib.name);
        let path = Path::new(&path);
        let str = serde_json::to_string(&lib)
            .unwrap_or_else(|e| panic!("Unable to serialize library {}: {e}", lib.name));
        std::fs::write(path, str)
            .unwrap_or_else(|e| panic!("Unable to write library {}: {e}", lib.name));
    }

    let symbols_555s = symbol_libraries.search_by_name("Timer, 555");
    for symbol in &symbols_555s {
        println!("Found symbol: {}", symbol.name);
    }
    schematic
        .place(symbols_555s[0], Position {
            x: PAGE_WIDTH as f32 / 2.,
            y: PAGE_HEIGHT as f32 / 2.,
            rotation: None,
        })
        .expect("Failed to place symbol");

    if let Some(output_path) = args.sheet_output {
        let mut file =
            OpenOptions::new().write(true).create(true).truncate(true).open(output_path).unwrap();
        file.write_all(schematic.to_string().as_bytes()).unwrap();
    }
}
