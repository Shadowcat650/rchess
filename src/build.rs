use crate::table_gen::{generate_tables, generate_zobrist};
use std::fs::File;
use std::path::Path;

mod defs;
mod mask_gen;
mod table_gen;

/// Generates computed tables at compile-time.
fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("generated_tables.rs");
    let mut tables = File::create(&path).unwrap();
    let path = Path::new(&out_dir).join("zobrist.rs");
    let mut zobrist = File::create(&path).unwrap();
    generate_tables(&mut tables);
    generate_zobrist(&mut zobrist);
}
