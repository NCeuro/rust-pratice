fn main() {
    let number = 29;

    if is_prime(number) {
        println!("{} це просте число", number);
    } else {
        println!("{} це не просте число", number);
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt = (n as f64).sqrt() as u32;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
