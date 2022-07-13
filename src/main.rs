use wasm_bindgen::prelude::*;
use web_sys::*;
mod parser;
mod dict_mapper;
mod vocab_list_constructor;

#[macro_use]
mod util;

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let subtitles_vector: Vec<String> = parser::parse_subs("spanish_subs.net");
    let (entry_map, entries) = dict_mapper::init_dict_map("conjugated_dict.txt");
   

    println!("{:?}", &subtitles_vector);
    // create vocab JSON list
    let vocab_list: String = vocab_list_constructor::new(subtitles_vector,entry_map,entries);

    println!("{}", vocab_list);
}
