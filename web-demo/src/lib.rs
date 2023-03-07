#![allow(clippy::unused_unit)] // weird clippy bug with wasm-bindgen
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_program(input: &str) -> String {
    let mut egraph = eqlog::EGraph::default();
    match egraph.parse_and_run_program(input) {
        Ok(outputs) => {
            log::info!("egg ok, {} outputs", outputs.len());
            outputs.join("<br>")
        }
        Err(e) => {
            log::info!("egg failed");
            e.to_string()
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(Default::default());
    console_error_panic_hook::set_once();
    log::info!("wasm initialized");
}
