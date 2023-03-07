use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

pub fn compress_contents_from_file() {

}

pub fn compress_contents_from_string(s: &str) -> String {
    return build_frequency_index(s);
}

fn build_frequency_index(text: &str) -> String {
    
    let mut freq_index = HashMap::<String, i32>::new();

    for g in text.graphemes(true) {
        match freq_index.get(g) {
            None => freq_index.insert(g.encode_utf16(), 1),
            Some(v) => freq_index.insert(g.encode_utf16(), v + 1)
        };
    }

    return build_string_from_index(freq_index);
}

fn build_string_from_index(map: HashMap::<String, i32>) -> String {
    let mut output = String::new();

    let mut vec: Vec<(String, i32)> = map.into_iter().collect();

    vec.sort_by(|a, b| (*b).1.cmp(&a.1));

    for tuple in &vec {
        output.push_str((tuple.0.clone() + ":" + &tuple.1.to_string() + ";").as_str());
    }

    return output;
}