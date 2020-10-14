fn main() {
    let mut count = 0u32;

    println!("{}", count);

    println!("let's count");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("it's enough");
            break;
        }
    }

    // -----------------------------------------------------

    let mut m = 1;
    let n = 1;

    'a: loop {
        if m < 100 {
            m += 1;
        } else {
            'b: loop {
                if m + n > 50 {
                    println!("break");
                    break 'a; // 可以在嵌套循环中操作任意循环
                } else {
                    continue 'b;
                }
            }
        }
    }

    // loop也可以像ifelse一样作为表达式
    let v = loop {
        break 10;
    };
    println!("{}", v);

    let v2 = loop {}; // 死循环，v2是发散类型，后面的代码永不会执行
}
