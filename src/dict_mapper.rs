use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

// struct for storing dictionary terms:
// root = spanish root
// translation = english translation
#[derive(Debug)]
pub struct DictEntry {
    pub root: String,
    pub translation: String,
    pub part_of_speech: String,
}
pub fn init_dict_map(filename: &str) -> (HashMap<String, String>, HashMap<String, DictEntry>) {
    // load in conjugated_dictionary.txt and read to string
    let mut file = File::open(filename).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");
   
    // entry_map is conjugated forms (and root form) -> root form
    // entries is root form -> DictEntry
    let mut entry_map: HashMap<String, String> = HashMap::new();
    let mut entries: HashMap<String, DictEntry> = HashMap::new();
    
    // all variables used to help make insertions correctly into hashmaps 
    let mut en_def = String::new();
    let mut pos = String::new();
    let mut es_root = String::new();
    let mut en_found = false;
    let mut pos_found = false;
    let mut root_found = false;
    for word in data.split_ascii_whitespace() {
        match word {
            "<en>" => {
                en_found = true;
                en_def = String::from("");
            },
            "</en>" => {
                en_found = false;
                en_def.pop(); // remove extra space at end
            },
            "<es>" => root_found = true,
            "<pos>" => {
                pos_found = true;
                pos = String::from("");
            },
            "</pos>" => {
                pos_found = false;
                pos.pop();
            },
            _ => {
                if en_found { 
                    // found en translation, note that it can be multiple words, we concatenate them
                    en_def.push_str(word);
                    en_def.push_str(" ");
                } else if pos_found {
                    pos.push_str(word);
                    pos.push_str(" ");
                } else if root_found { 
                    // found es root form, store in es_root
                    root_found = false;
                    es_root = String::from(word);
                    // initialize entry to be added to entries hashmap
                    let entry = DictEntry {
                        root: es_root.clone(),
                        translation: en_def.clone(),
                        part_of_speech: pos.clone(),
                    };
                    // hashmap insertions
                    if entries.contains_key(&es_root) {
                        let mut comma = String::from(", ");
                        comma.push_str(&en_def);
                        entries.get_mut(&es_root)
                            .unwrap()
                            .translation
                            .push_str(&comma);
                    } else {
                        entries.insert(
                            es_root.clone(),
                            entry,
                        );
                    }
                    // we clone here since we need to retain the root for future insertions
                    entry_map.insert(
                        es_root.clone(),
                        es_root.clone(),
                    );
                } else { 
                    // reached conjugated forms, we only need to map them to root in entry_map
                    entry_map.insert(
                        word.to_string(),
                        es_root.clone(),
                    );
                }
            },
        } 
    }
    (entry_map, entries)
}
