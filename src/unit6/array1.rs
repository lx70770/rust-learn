fn main() {
    fn mut_array(a: &mut [i32]) {
        a[2] = 5;
    }

    println!("size of &[i32; 3]: {:?}", std::mem::size_of::<&[i32; 3]>());
    println!("size of &[i32]: {:?}", std::mem::size_of::<&[i32]>());

    let mut v: [i32; 3] = [1, 2, 3];
    {
        let s: &mut [i32; 3] = &mut v;
        mut_array(s); // 数组切片可以修改原数组
    }

    println!("{:?}", v);
}