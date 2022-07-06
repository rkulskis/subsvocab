use wasm_bindgen::prelude::*;
use web_sys::*;
mod parser;

#[macro_use]
mod util;

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // log!("Hello World!");
    parser::parse_subs("ipv4-c093-bos001-ix.1.oca.nflxvideo.net");
}