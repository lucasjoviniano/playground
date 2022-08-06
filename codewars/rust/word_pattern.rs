use std::collections::HashMap;

fn word_pattern(w: &str) -> String {
    let mut map = HashMap::new();
    let word = w.to_lowercase().to_string();
    let mut index: usize = 0;

    for c in word.chars() {
        if let None = map.get(&c) {
            map.insert(c, index);
            index += 1;
        }
    }

    word.chars().map(|c| *map.get(&c).unwrap()).collect::<Vec<usize>>().iter().map(|&c| c.to_string()).collect::<Vec<String>>().join(".")
}

fn main() {
    let s = "hello";

    let result = word_pattern(&s);
}
