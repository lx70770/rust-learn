fn main() {
    let v = [10i32, 20, 30, 40, 50];
    let first = v.get(0);
    let second = v.get(3).is_none();

    println!("{:?}, {:?}", first, second);
}