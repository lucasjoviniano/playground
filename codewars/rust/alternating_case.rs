fn to_alternating_case(s: &str) -> String {
    s
        .chars()
        .map(|c| if c.is_lowercase() { return c.to_uppercase().to_string() } else { return c.to_lowercase().to_string() })
        .collect()
}

fn main() {
    assert_eq!("HELLO WORLD", to_alternating_case("hello world"));
    assert_eq!("hello world", to_alternating_case("HELLO WORLD"));
    assert_eq!("HELLO world", to_alternating_case("hello WORLD"))    assert_eq!("HELLO WORLD", to_alternating_case("hello world"));
    assert_eq!("hello world", to_alternating_case("HELLO WORLD"));
    assert_eq!("HELLO world", to_alternating_case("hello WORLD"));
    assert_eq!("hEllO wOrld", to_alternating_case("HeLLo WoRLD"));
    assert_eq!("Hello World", to_alternating_case(&to_alternating_case("Hello World")[..]));
    assert_eq!("12345", to_alternating_case("12345"));
    assert_eq!("1A2B3C4D5E", to_alternating_case("1a2b3c4d5e"));
    assert_eq!("sTRING.tOaLTERNATINGcASE", to_alternating_case("String.ToAlternatingCase"));;
    assert_eq!("hEllO wOrld", to_alternating_case("HeLLo WoRLD"));
    assert_eq!("Hello World", to_alternating_case(&to_alternating_case("Hello World")[..]));
    assert_eq!("12345", to_alternating_case("12345"));
    assert_eq!("1A2B3C4D5E", to_alternating_case("1a2b3c4d5e"));
    assert_eq!("sTRING.tOaLTERNATINGcASE", to_alternating_case("String.ToAlternatingCase"));
}
