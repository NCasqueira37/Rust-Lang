use std::mem;

fn main() {
    variables();
    mutvar();
    array();
    tuple();
    literals();

    let tup=(5,5);
    println!("Printing a tuple passed into a function {}", passingtuple(tup));

    printarraysize();
}

fn variables(){
    let x: i32 = 3;
    let y: i32 = 7;
    let b: bool = true;
    let c: char = 'n';
    println!("x: {}, y: {}", x, y);
    println!("bool: {}, char: {}", b, c);
}

fn mutvar(){
    let mut x: i32 = 10;
    println!("Mutable variable: {}", x);
    x = 20;
    println!("Mutable variable: {}", x);
}

fn array(){
    let arr: [i32; 5] = [213,1213,543,114,76];
    print!("Array: {} ", arr[0]);
    print!("Array: {} ", arr[1]);
    print!("Array: {} ", arr[2]);
    print!("Array: {} ", arr[3]);
    print!("Array: {} ", arr[4]);
    println!("");
}

fn tuple(){
    // let tup = (true, 72, 'a', 3.14);
    let tup: (bool, i32, char, f32) = (true, 72, 'a', 3.14);
    print!("Tuple: {} ", tup.0);
    print!("Tuple: {} ", tup.1);
    print!("Tuple: {} ", tup.2);
    print!("Tuple: {} ", tup.3);
    println!("");
}

fn literals(){
    println!("int: {}", 1 + 2);
    // Can use underscores to improve readability
    println!("million: {}", 1_000_000u32);
}

fn passingtuple(tup: (i32, i32)) -> i32 {
    tup.0 + tup.1
}

fn printarraysize(){
    let arr: [i32; 500]= [10; 500];
    println!("Size is: {}", mem::size_of_val(&arr));
}