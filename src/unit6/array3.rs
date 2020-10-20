use std::ops::Range;

fn main() {
    let r = 1..10; // 类似 [1 - 10) 不包括10
    for i in r {
        println!("{:?}\t", i);
    }

    // r 等同于
    let _r = Range { start: 1, end: 10 };

    // js array.map
    let r3 = (1i32..11).rev().map(|i| i * 10);

    for i in r3 {
        print!("{:?}\t", i);
    }
}