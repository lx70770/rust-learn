fn f1() -> bool {
    println!("call f1");
    true
}

fn f2() -> bool {
    println!("call f2");
    false
}

fn main() {
    println!("Bit and: {}\n", f2() & f1()); // 按位与
    println!("Bit and: {}\n", f2() && f1()); // 逻辑与
    println!("Logic or: {}\n", f1() | f2()); // 按位或
    println!("Logic or: {}\n", f1() || f2()); // 按位与
}
