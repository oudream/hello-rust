fn main() {
    // hello_primitive_type3();
    // slice_out_of_array();
    hello_tuple();
}

#[test]
fn indexing_tuple() {
    // let numbers = [1, 2, 3];
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    // let second = numbers[1];
    let second = numbers.1;

    assert_eq!(2, second,
               "This is not the 2nd number in the tuple!")
}

fn hello_tuple() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; // 解构元组到变量 name 和 age

    println!("{} is {} years old.", name, age);
}

fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // 获取索引为 1 到 3 的切片（左闭右开）

    assert_eq!([2, 3, 4], nice_slice);
}

fn hello_primitive_type3() {
    let a = vec![0; 100]; // 创建包含 100 个元素，初始值为 0 的数组

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}

fn hello_primitive_type2() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '1';// Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

fn hello_primitive_type1() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = true;// Finish the rest of this line like the example! Or make it be false!
    if is_evening {
        println!("Good evening!");
    }
}