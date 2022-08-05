#![feature(exclusive_range_pattern)]
fn number_to_words(n: usize) -> String {
    let ones = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let tens = vec![
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    match n {
        0..20 => return ones[n].to_string(),
        20..=90 if n % 10 == 0 => return tens[n / 10].to_string(),
        20..100 => return tens[n / 10].to_owned() + ones[n % 10],
        100..=900 if n % 100 == 0 => return ones[n / 100].to_owned() + "hundred",
        100..1000 => {
            return ones[n / 100].to_owned() + "hundredand" + number_to_words(n % 100).as_str()
        }
        1000 => return String::from("onethousand"),
        _ => return String::new(),
    }
}

fn main() {
    let target = 1000;
    let mut answer = 0;

    for i in 0..target {
        let word = number_to_words(i + 1);
        answer += word.len();
    }

    println!("{}", answer);
}
