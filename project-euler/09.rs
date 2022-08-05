fn main() {
    'outer: for i in 1..1000 {
        for j in 1..1000 {
            for k in  1..1000 {
                if i + j + k == 1000 && is_triplet(i, j, k) {
                    println!("{}", i * j * k);
                    break 'outer;
                }
            }
        }
    }
}

fn is_triplet(a: u64, b: u64, c: u64) -> bool {

    if a.pow(2) + b.pow(2) == c.pow(2) {
        return true;
    }

    false
}