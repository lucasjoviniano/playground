fn solution(word: &str, ending: &str) -> bool {
    let start = word.len() - ending.len();
    let end = word.len();

    &word[start..end] == ending
}
