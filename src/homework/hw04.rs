fn main() {
    const HEIGHT: usize = 11;
    const WIDTH: usize = HEIGHT; 

    let mut canvas = vec![vec![' '; WIDTH]; HEIGHT];
    let mid = HEIGHT / 2;

    for y in 0..HEIGHT {
        let stars = if y <= mid {
            1 + 2 * y
        } else {
            1 + 2 * (HEIGHT - 1 - y)
        };
        let spaces = (WIDTH - stars) / 2;

        for x in 0..stars {
            canvas[y][spaces + x] = '*';
        }
    }

    for row in canvas {
        print!("{}", row.iter().collect::<String>());
        println!();
    }
}
