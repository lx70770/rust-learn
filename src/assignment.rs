fn main() {
    let x = 1;
    let mut y = 2;

    let z = (y = x); // 编译出错，深入浅出Rust上说z的类型是空的元祖tuple
    println!("{:?}", z);
}
