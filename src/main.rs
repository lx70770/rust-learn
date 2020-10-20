// use ferris_says::say;
// use std::io::{stdout, BufWriter};

// mod statement_expression;

fn main() {
    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();

    // // 空元祖占用内存空间为0
    // println!("size of i8 {}", std::mem::size_of::<i8>());
    // println!("size of char {}", std::mem::size_of::<char>());
    // println!("size of '()' {}", std::mem::size_of::<()>());

    // // 结构体
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }

    // let x = 10;
    // let y = 20;

    // let p = Point { x, y };
    // println!("Point is at {} {}", p.x, p.y);

    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    fn default() -> Point3d {
        Point3d { x: 0, y: 0, z: 0 }
    }

    // ..语法可以使用别的构造体内的部分成员
    let origin = Point3d { ..default() };
    let point = Point3d {
        z: 1,
        x: 3,
        ..origin
    };

    println!("origin is {} {} {}", origin.x, origin.y, origin.z);
    println!("point is {} {} {}", point.x, point.y, point.z);
}
