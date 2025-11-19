use kicad_generator::schematic::{symbol_library::SymbolLibrary, KicadSch};

use clap::Parser;
use lazy_static::lazy_static;
use std::sync::OnceLock;

lazy_static! {
    static ref KICAD_3RD_PARTY_PATH: OnceLock<String> = OnceLock::new();
    static ref KICAD_SYMBOL_LIBRARIES_PATH: OnceLock<String> = OnceLock::new();
}

// A program to parse, modify generate KiCad schematics
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    project_directory: String,

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
    KICAD_3RD_PARTY_PATH
        .set(path)
        .expect("Should be the first time setting KICAD_3RD_PARTY_PATH");
    KICAD_SYMBOL_LIBRARIES_PATH
        .set(args.kicad_symbol_path)
        .expect("Should be the first time setting KICAD_SYMBOL_LIBRARIES_PATH");

    let _schematic = KicadSch::default();

    let _symbol_lib = SymbolLibrary::from_path(format!(
        "{}/com_github_CDFER_JLCPCB-Kicad-Library/JLCPCB-ICs.kicad_sym",
        KICAD_3RD_PARTY_PATH.get().unwrap()
    ))
    .unwrap_or_else(|e| panic!("{}", e));
}
