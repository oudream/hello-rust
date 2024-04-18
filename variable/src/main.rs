
// https://yanglixin.com/posts/program/rust/basic/global-variable.html

use std::sync::{Mutex, Once};


fn main() {
    hello_variable11();
    // hello_global_variable3();
}


// 定义全局变量
static mut COUNTER: Option<Mutex<i32>> = None;
static INIT: Once = Once::new();

// 初始化全局变量
fn init_counter() {
    unsafe {
        COUNTER = Some(Mutex::new(0));
    }
}

fn hello_global_variable3() {
    // 初始化全局变量
    INIT.call_once(|| init_counter());

    // 在多个线程中修改计数器的值
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = std::thread::spawn(move || {
            // 获取锁，修改计数器的值
            unsafe {
                let counter = COUNTER.as_ref().unwrap();
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印计数器的最终值
    unsafe {
        let counter = COUNTER.as_ref().unwrap();
        println!("Final counter value: {}", *counter.lock().unwrap());
    }
}


// 原子类型，线程安全性
use std::sync::atomic::{AtomicUsize, Ordering};
static REQUEST_RECV_2: AtomicUsize = AtomicUsize::new(0);

fn hello_global_variable2() {
    for _ in 0..100 {
        REQUEST_RECV_2.fetch_add(1, Ordering::Relaxed);
    }
    println!("current request recv: {}", REQUEST_RECV_2.load(Ordering::Relaxed));
}

// 静态变量
static mut REQUEST_RECV_1: i64 = 0;
fn hello_global_variable1() {
    unsafe {
        REQUEST_RECV_1 += 1;
        assert_eq!(REQUEST_RECV_1, 1);
    }
}



fn hello_variable11() {
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);                       // s 的值移动到函数里 ...
    // println!("{}", s); // 编译出错，所以到这里不再有效

    let x = 5;                           // x 进入作用域
    makes_copy(x);                            // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string); } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer); } // 这里，some_integer 移出作用域。不会有特殊操作



// 克隆(深拷贝)
// Rust 永远也不会自动创建数据的 “深拷贝”
fn hello_variable10() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}



// 引用与借用
fn hello_variable9() {
    // x 只是引用了存储在二进制可执行文件( binary )中的字符串 "hello, world"，并没有持有所有权。
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);
    // 借用的错误使用：
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1); // value borrowed here after move
}



const NUMBER: i32 = 3;
fn hello_variable8() {
    println!("Number {}", NUMBER);
}



fn hello_variable7() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}



fn hello_variable6() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}



fn hello_variable5() {
    let x: i8 = 10;
    println!("Number {}", x);
}



fn hello_variable4() {
    let mut x = 5;
    let y = &mut x; // 创建一个可变引用指向变量x的值
    *y = 10; // 使用解引用操作符*来修改引用的值
    println!("x 修改后的值为: {}", x);
}



fn hello_variable3() {
    let x = 5;
    let y = &x; // 创建一个不可变引用指向变量x的值
    println!("x 的值为: {}", *y); // 使用解引用操作符*来访问引用的值
}



fn hello_variable2() {
    let x = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}



fn hello_variable1() {
    let x = 5;
    println!("x has the value {}", x);
}

