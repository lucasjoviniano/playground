fn main() {
    let max = 2000000;

    let mut v: Vec<u64> = Vec::new();

    v.push(2);

    for i in (3..=max).step_by(2) {
        let mut is_prime = true;

        for p in v.iter() {
            if p * p > i {
                break
            }

            if i % p == 0 {
                is_prime = false;
                break
            }
        }

        if is_prime {
            v.push(i)
        }
    }

    let sum: u64 = v.iter().sum();

    println!("{}", sum)
}