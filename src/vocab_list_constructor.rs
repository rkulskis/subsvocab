use std::collections::HashMap;
use crate::dict_mapper::DictEntry;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct VocabEntry {
    es_root: String,
    part_of_speech: String,
    en_translation: String,
    count: u32,
}
pub fn new(subtitles_vector: Vec<String>, entry_map: HashMap<String, String>, entries: HashMap<String, DictEntry>) -> String {
    let mut vocab_list: HashMap<String, VocabEntry> = HashMap::new();
    for word in subtitles_vector {
        vocab_list_insert(word, &mut vocab_list, &entries, &entry_map);
    }
    let mut vocab_vector: Vec<VocabEntry> = Vec::new();
    for entry in vocab_list.values() {
        let entry_copy = VocabEntry {
            es_root: entry.es_root.to_owned(),
            en_translation: entry.en_translation.to_owned(),
            part_of_speech: entry.part_of_speech.to_owned(),
            count: entry.count,
        };
        vocab_vector.push(entry_copy);
    }
    vocab_vector.sort_by(|a, b| b.count.cmp(&a.count));
    let json_vocab = serde_json::to_string_pretty(&vocab_vector).unwrap();
    json_vocab
}

fn vocab_list_insert(word: String, vocab_list: &mut HashMap<String, VocabEntry>, entries: &HashMap<String, DictEntry>, entry_map: &HashMap<String, String>) {
    // this iterator thing was designed to allow multiple interpretations
    // of same word (i.e. it matches conjugations of multiple words)
    // however, didn't work (for now) but fix this or reduce later
    let keys = entry_map.get(&word).to_owned();
    if keys != None {
        let keys = keys.unwrap(); 
        // key = a spanish root word
        for key in keys.split_ascii_whitespace() {
            if key == "OR" {
                continue;
            }
            let entry = entries.get(key).unwrap(); 
            if vocab_list.contains_key(key) {
                vocab_list.get_mut(key).unwrap().count = vocab_list.get(key).unwrap().count + 1;
            } else {
                let vocab_word = VocabEntry {
                    es_root: entry.root.to_owned(),
                    en_translation: entry.translation.to_owned(),
                    part_of_speech: entry.part_of_speech.to_owned(),
                    count: 1,
                };
                vocab_list.insert(key.to_string(), vocab_word);
            }
        }
    }
}
