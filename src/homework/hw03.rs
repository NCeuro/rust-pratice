fn main() {
    const W: usize = 40;
    const H: usize = 20; 

    let mut canvas = vec![vec![' '; W]; H];

    // контур
    for x in 0..W {
        canvas[0][x] = '*';
        canvas[H - 1][x] = '*';
    }
    for y in 0..H {
        canvas[y][0] = '*';
        canvas[y][W - 1] = '*';
    }

    // діагоналі (з'єднують кути — як "зашивка" конверта)
    for i in 0..H {
        let left_to_right_x = i * (W - 1) / (H - 1);
        let right_to_left_x = (W - 1) - left_to_right_x;

        if left_to_right_x < W {
            canvas[i][left_to_right_x] = '/';
        }
        if right_to_left_x < W {
            canvas[i][right_to_left_x] = '\\';
        }
    }

    // виведення малюнку
    for row in &canvas {
        print!("{}", row.iter().collect::<String>());
        println!();
    }
}
