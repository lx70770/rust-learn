fn print_slice(arr: &[i32]) {
    println!("Length: {}", arr.len());

    for item in arr {
        println!("{}\t", item);
    }
}

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    print_slice(&arr[..]);

    let slice = &arr[2..];
    print_slice(slice);

    let slice2 = &slice[..2];
    print_slice(slice2);

    // 以上切片都是右边开区间
    // 以下是左右闭区间

    let slice3 = &arr[1..=2];
}