fn main() {
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
