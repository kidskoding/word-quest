use std::fs::File;
use std::io;

pub fn get_available_words() -> Vec<String> {
    let path = "./src/words_dictionary.json";
    let file = File::open(path).expect(
        &*("Could not open file: ".to_owned() + path)
    );
    let reader = io::BufReader::new(file);
    let words_map: std::collections::HashMap<String, i32> = serde_json::from_reader(reader).expect(
        "Could not parse JSON file"
    );
    
    words_map.keys().cloned().collect()
}
