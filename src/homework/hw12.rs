use rand::Rng;

// Основна функція: підрахунок кількості переміщень
fn count_permutation(shipments: &Vec<u32>) -> Result<usize, &'static str> {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return Err("неможливо рівномірно розподілити вантаж між суднами");
    }

    let target = total / n as u32;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments {
        balance += weight as isize - target as isize;
        moves += balance.abs();
    }

    Ok((moves / 2) as usize)
}

// Генератор коректних shipment-ів
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(10..100);
    let total = target * n as u32;
    let mut shipments = vec![0u32; n];

    for i in 0..n - 1 {
        let max_val = total - shipments[..i].iter().sum::<u32>() - (n - i - 1) as u32;
        shipments[i] = rng.gen_range(1..=max_val);
    }

    let current_sum: u32 = shipments.iter().sum();
    shipments[n - 1] = total - current_sum;
    shipments
}

// Приклади з поясненням
fn main() {
    let example1 = vec![1, 1, 1, 1, 6];
    println!("приклад 1: {:?} => {:?}", example1, count_permutation(&example1));

    let example2 = vec![1, 1, 1, 6];
    println!("приклад 2: {:?} => {:?}", example2, count_permutation(&example2));

    let generated = gen_shipments(5);
    println!("вантажі: {:?} => {:?}", generated, count_permutation(&generated));
}
