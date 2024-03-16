// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.
// Execute `rustlings note options3` or use the `note options3` watch subcommand for lzz's note.



// I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
