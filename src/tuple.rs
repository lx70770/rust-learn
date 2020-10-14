fn main() {
    let a = (1i32, false);
    let b = ('a', (1i32, 1i32));

    println!(a.0, a.1);

    // 空元祖占用内存空间为0
    println!("size of i8 {}", std::mem::size_of::<i8>());
    println!("size of char {}", std::mem::size_of::<char>());
    println!("size of '()' {}", std::mem::size_of::<()>());
}
