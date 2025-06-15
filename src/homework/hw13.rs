#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[derive(Debug)]
struct Event {
    x: i32,
    y1: i32,
    y2: i32,
    typ: i32, // 1 - відкриття, -1 - закриття
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut events = vec![];

    for rect in xs {
        let x1 = rect.a.x.min(rect.b.x);
        let x2 = rect.a.x.max(rect.b.x);
        let y1 = rect.a.y.min(rect.b.y);
        let y2 = rect.a.y.max(rect.b.y);

        events.push(Event { x: x1, y1, y2, typ: 1 });
        events.push(Event { x: x2, y1, y2, typ: -1 });
    }

    events.sort_by_key(|e| e.x);

    let mut active = vec![];
    let mut prev_x = events[0].x;
    let mut total_area = 0;

    for event in events {
        let dx = event.x - prev_x;

        if dx > 0 {
            let mut intervals = active.clone();
            intervals.sort();

            let mut covered_y = 0;
            let mut current_start = -1;
            let mut current_end = -1;

            for (y1, y2) in intervals {
                if y1 > current_end {
                    covered_y += current_end - current_start;
                    current_start = y1;
                    current_end = y2;
                } else {
                    current_end = current_end.max(y2);
                }
            }

            covered_y += current_end - current_start;
            total_area += covered_y * dx;
        }

        if event.typ == 1 {
            active.push((event.y1, event.y2));
        } else {
            if let Some(pos) = active.iter().position(|&(a, b)| a == event.y1 && b == event.y2) {
                active.remove(pos);
            }
        }

        prev_x = event.x;
    }

    total_area
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Загальна зайнята площа: {}", occupied);
}

