use std::collections::HashSet;

fn longest_codewars(a1: &str, a2: &str) -> String {
    format!("{}{}", a1, a2).chars().sorted().dedup().collect()
}

fn longest(a1: &str, a2: &str) -> String {
    let mut set = HashSet::new();
    let str1 = a1.chars().collect::<Vec<char>>();
    let str2 = a2.chars().collect::<Vec<char>>();

    for i in str1 {
        set.insert(i);
    }

    for j in str2 {
        set.insert(j);
    }

    let mut sorted: Vec<char> = Vec::new();

    for i in &set {
        sorted.push(*i);
    }
    sorted.sort();

    sorted.iter().collect()
}

fn main() {
    println!("{}", longest("xyaabbbccccdefww", "xxxxyyyyabklmopq"));
}
