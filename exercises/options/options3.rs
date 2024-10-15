// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
     // Fix without deleting this line.
    if let Some( ref p) = y{
        println!("Point exists with coordinates: ({},{})", p.x, p.y);
    } else {
        println!("No point available");
    }
}
