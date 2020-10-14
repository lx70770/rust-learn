fn main() {
    let x: () = {
        println!("Hello.");
    };

    let y: i32 = {
        println!("Hello.");
        5
    };

    println!("{:?}", x); // 打印() tuple
    println!("{:?}", y); // 5
}
