use std::vec;

pub mod vector_fn;

pub fn learn_vec(){

    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    println!("In this file we will learn about vector");
    vector_fn::some_vector_fn();

    println!("Print the value of vector v {:?}", v);
}