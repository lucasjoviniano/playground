fn main() {
    let mut max = 0;

    for i in 1..999 {
        for j in 1..999 {
            let p = i * j;
            if is_palindrome(p) && p > max {
                max = p;
            }
        }
    }

    println!("{}", max)
}

fn is_palindrome(i: u32) -> bool {
    let mut reversed: u32 = 0;
    let mut t = i;

    while t > 0 {
        reversed = reversed * 10 + (t % 10);
        t/= 10;
    }

    reversed == i
}