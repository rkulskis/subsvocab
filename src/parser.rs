extern crate unidecode;
use unidecode::unidecode;
use std::fs::File;
use std::io::Read;
use scraper::{Html, Selector};

// this function accepts a slice representing the location of a .net subtitles file
// and returns a vector of Strings such that each element is a word
// in the subtitle script
pub fn parse_subs(filename: &str) -> Vec<String> {
    // load in subs file and read to string
    let mut file = File::open(filename).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");

    let data_offset = data.find("<body>").unwrap_or(data.len());
    
    data.replace_range(..data_offset, "");
    let document = Html::parse_document(&data);

    // get subtitles and collect into String
    let subs: String = document
        .select(&Selector::parse("body div p span").unwrap())
        .flat_map(|el| el.text().map(|t| t.to_string() + " "))
        .collect();

    // parse string, removing all digits, punctuation, and extra spaces
    // collect into Vec<&str>
    let subs = subs
        .replace(&['(', ')', ',', '\"', '.', ';', ':', '\'','¿','?','\\','—','-','!','¡', '#','0','1','2','3','4','5','6','7','8','9'], " ");

    // create vector of Strings from subtitles and cast to ASCII using unidecode (removes
    // accentuation). also make all lowercase 
    let subtitles_vector: Vec<String> = subs
        .split_ascii_whitespace()
        .map(|s| unidecode(s).to_lowercase()) 
        .collect();

    subtitles_vector
}
