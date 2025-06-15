use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32, i32)> {
    if data.len() < 2 {
        return None;
    }
    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }
    Some((min_index, data[min_index], data[min_index + 1]))
}

fn print_vector_with_min_pair(data: &[i32]) {
    let len = data.len();

    // Друкуємо індекси
    print!("indexes:");
    for i in 0..len {
        print!(" {:>2}.", i);
    }
    println!();

    // Друкуємо data
    println!("data:   {:?}", data);

    if let Some((min_idx, val1, val2)) = min_adjacent_sum(data) {

        let pos = 9 + min_idx * 4;

        let mut marker = String::new();
        for _ in 0..pos {
            marker.push(' ');
        }
        marker.push_str(r"\__ __/");

        println!("indexes:{}", marker);

        println!(
            "min adjacent sum={}+{}={} at indexes:{},{}",
            val1,
            val2,
            val1 + val2,
            min_idx,
            min_idx + 1
        );
    } else {
        println!("Vector is too short to find adjacent pairs.");
    }
}

fn main() {
    for _ in 0..4 {
        let vec = gen_random_vector(20);
        print_vector_with_min_pair(&vec);
        println!();
    }
}
