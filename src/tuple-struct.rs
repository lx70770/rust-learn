fn main() {
    // tuple struct有名字但是没有成员名字
    struct Color(i32, i32, i32);

    // 等价于

    // struct Color {
    //     0: i32,
    //     1: i32,
    //     2: i32,
    // }
}
