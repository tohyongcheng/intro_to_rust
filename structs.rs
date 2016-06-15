// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f64);

// A struct with two fields
struct Point {
    x: f64,
    y: f64,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(r:Rectangle) -> f64 {
    let x_change = (r.p1.x - r.p2.x).abs();
    let y_change = (r.p1.y - r.p2.y).abs();

    x_change * y_change
}

fn square(p:Point, height:f64) -> Rectangle {
    Rectangle {
        p1: Point { x: p.x, y: p.y },
        p2: Point { x: (p.x + height), y: (p.y + height) }
    }
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("The rectangle area is {}", rect_area(_rectangle));

    square(Point { x: 1.0, y: 2.0}, 2.0);
}
