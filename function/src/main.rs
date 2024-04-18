fn main() {
    // hello_fun2(3);

    // let original_price = 51;
    // println!("Your sale price is {}", hello_fun4(original_price));

    // let answer = hello_fun5(3);
    // println!("The square of 3 is {}", answer);
}

fn hello_fun5(num: i32) -> i32 {
    num * num
}

fn hello_fun4(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}


fn hello_fun3(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn hello_fun2(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn hello_fun1() {
    // todo!()
}
