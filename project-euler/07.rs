fn main() {
    let mut ans: u64 = 0;
    let mut i: u64 = 0;

    while i < 10001 {
        ans += 1;
        if is_prime(ans) {
            i += 1;
        }
    }

    println!("{}", ans)
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}