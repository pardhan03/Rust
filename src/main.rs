mod utils;

use utils::learn_enum::{log_direction, calculate};
use Rust::stack_fn;
use Rust::heap_fn;
use Rust::update_string;
use Rust::heap_own;
use Rust::string_own;

use Rust::learn_vec;

use Rust::hash_map;

// use Rust::learn_generics;
use  Rust::learn_generics;

use Rust::learn_trait;
fn main() {

    let sentence =String::from("This is my first Rust program");

    let first_word=get_first_word(sentence);

    println!("First word of the String is {}", first_word);

    let string= String::from("Hello world");
    println!("{}",string);
    println!("Hello, world!");

    stack_fn(); // call the function that uses stack memory
    heap_fn(); // call the function that uses heap memory
    update_string(); // call the function that changes size of variable at runtime

    heap_own();

    string_own();

    log_direction();

    calculate();

    utils::create_user();

    utils::test_rectangle();

    learn_vec::learn_vec();

    hash_map::learn_hashmap();

    learn_generics::main_gen_fun();

    learn_trait::trait_fn();

    let result = match divide(4, 0){
        Ok(num) => num,
        Err(err) => {
            println!("Error {err}");
            -1
        }
    };

    println!("Get the result of the divide function {:?}", result);
}

fn get_first_word(sentence: String) -> String {
    let mut ans=String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return  ans;
}

pub fn divide(x:i32, y:i32) -> Result<i32, String>{
    if y ==0 {
        return Err(String::from("Please don't divided by 0"));
    }

    Ok(x/y)
}