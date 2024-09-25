pub fn assign_error_example(text: &mut String) {
    let mut x = 5;
    println!("{text}: Num is {x}");
    x = 10;
    println!("{text}: Num is {x}");
    loop_test();
}

fn loop_test() {
    for x in 1..=2 {
        println!("{x}");
    }
    let mut ch = 0;
    'outer_mark: loop {
        ch += 1;
        println!("{ch}");
        if ch == 6 { break; }
        loop {
            ch += 1;
            if ch == 5 { break 'outer_mark; }
            println!("{ch}");
        }
    }
}

