use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

// Функція, яка рахує фактичну зайняту площу
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut points: HashSet<Point> = HashSet::new();

    for rect in xs {
        let x_min = std::cmp::min(rect.a.x, rect.b.x);
        let x_max = std::cmp::max(rect.a.x, rect.b.x);
        let y_min = std::cmp::min(rect.a.y, rect.b.y);
        let y_max = std::cmp::max(rect.a.y, rect.b.y);

        for x in x_min..x_max {
            for y in y_min..y_max {
                points.insert(Point { x, y });
            }
        }
    }

    points.len() as i32
}

// Тестові дані
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

// Функція тестування
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Тест пройдено успішно.");
}
