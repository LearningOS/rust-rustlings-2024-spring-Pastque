// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // 第一段代码是对Option<Point>的引用进行模式匹配，不需要显式使用ref关键字；
    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    // 第二段代码直接对Option<Point>进行模式匹配，并在模式中显式创建了一个对内部Point结构体的引用；
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }

    // 但如果后续还需要在match之外的地方使用y的副本或者所有权，
    // 则应考虑选择不会移动所有权的方式，即第一种方法（匹配引用）。
    // 若不需要保留y的所有权，则两者皆可。 

    y; // Fix without deleting this line.
}
