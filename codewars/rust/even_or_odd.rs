fn even_or_oddd(i: i32) -> &'static str {
    if i % 2 == 0 {
        return "Even"
    }
    
    "Odd"
}
