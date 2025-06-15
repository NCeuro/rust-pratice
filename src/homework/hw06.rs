fn main() {
    let triangles = 5;
    draw_tree(triangles);
}

fn draw_tree(triangles: usize) {
    let max_width = 1 + 2 * (triangles + 1); // ширина останнього трикутника

    (1..=triangles).for_each(|level| {
        for i in 0..=level {
            let stars = 1 + 2 * i;
            let spaces = (max_width - stars) / 2;

            let line: String = std::iter::repeat(' ')
                .take(spaces)
                .chain(std::iter::repeat('*').take(stars))
                .collect();

            println!("{}", line);
        }
    });
}
